use crate::emitter::BundleIndexes;
use crate::model::XrefEntry;
use crate::registers::infer_peripheral_name;
use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Builds a deduplicated Hardware Cross-Reference Graph (`Vec<XrefEntry>`) linking
/// peripherals, registers, bitfields, table rows (pins, memory regions, interrupts, electrical specs),
/// figures, and chapters across the document.
pub fn build_xref_graph(indexes: &BundleIndexes) -> Vec<XrefEntry> {
    let mut entries: Vec<XrefEntry> = Vec::new();
    let mut seen: HashSet<(String, String, String)> = HashSet::new();

    let mut push_unique = |entry: XrefEntry| {
        let key = (
            entry.symbol.to_ascii_uppercase(),
            entry.kind.clone(),
            entry.location.clone(),
        );
        if seen.insert(key) {
            entries.push(entry);
        }
    };

    // 1. Index sections and inferred peripherals
    for sec in &indexes.sections {
        let periph = infer_peripheral_name(&sec.title);
        if periph != "SYS" && periph.len() >= 2 {
            push_unique(XrefEntry {
                symbol: periph.clone(),
                kind: "section".to_string(),
                peripheral: periph.clone(),
                summary: format!(
                    "Chapter {}: {} (pp. {}-{}, {} regs, {} tables)",
                    sec.section_id,
                    sec.title,
                    sec.start_page,
                    sec.end_page,
                    sec.register_count,
                    sec.table_count
                ),
                location: format!("{}:1", sec.file_rel_path),
                page_number: sec.start_page,
            });
        }
    }

    // 2. Index unique registers and their bitfields
    let mut seen_reg_headers: HashSet<(String, String, String)> = HashSet::new();
    for bf in &indexes.registers {
        let periph = if bf.peripheral == "SYS" || bf.peripheral.is_empty() {
            if let Some((prefix, _)) = bf.register.split_once('_') {
                let cleaned = prefix.trim_end_matches(['x', 'n', 'y']);
                if cleaned.len() >= 2 && cleaned.chars().all(|c| c.is_ascii_alphanumeric()) {
                    cleaned.to_ascii_uppercase()
                } else {
                    bf.peripheral.clone()
                }
            } else {
                bf.peripheral.clone()
            }
        } else {
            bf.peripheral.clone()
        };

        let reg_loc = format!("{}:{}", bf.section_file, bf.line_number.saturating_sub(2));
        let reg_key = (periph.clone(), bf.register.clone(), bf.section_file.clone());
        if seen_reg_headers.insert(reg_key) {
            push_unique(XrefEntry {
                symbol: bf.register.clone(),
                kind: "register".to_string(),
                peripheral: periph.clone(),
                summary: format!(
                    "{}.{} (Offset = {}, Reset = {})",
                    periph, bf.register, bf.offset, bf.reset_value
                ),
                location: reg_loc.clone(),
                page_number: bf.page_number,
            });

            if periph != "SYS" && !periph.is_empty() {
                push_unique(XrefEntry {
                    symbol: format!("{}.{}", periph, bf.register),
                    kind: "register".to_string(),
                    peripheral: periph.clone(),
                    summary: format!("Offset = {}, Reset = {}", bf.offset, bf.reset_value),
                    location: reg_loc,
                    page_number: bf.page_number,
                });
            }
        }

        if bf.field_name != "RESERVED" && bf.field_name.len() >= 2 {
            push_unique(XrefEntry {
                symbol: bf.field_name.clone(),
                kind: "bitfield".to_string(),
                peripheral: periph.clone(),
                summary: format!(
                    "{}.{}[{}] ({}, reset={}): {}",
                    periph,
                    bf.register,
                    bf.bits,
                    bf.access,
                    bf.field_reset,
                    truncate_summary(&bf.description, 90)
                ),
                location: format!("{}:{}", bf.section_file, bf.line_number),
                page_number: bf.page_number,
            });
        }
    }

    // 3. Index hardware symbols inside non-register tables (Pinouts, Memory Maps, Interrupts, Electrical Specs)
    static RE_HW_TOKEN: OnceLock<Regex> = OnceLock::new();
    let re_hw = RE_HW_TOKEN.get_or_init(|| {
        Regex::new(
            r"\b(0x[0-9A-Fa-f]{4}(?:[._][0-9A-Fa-f]{4})?|0x[0-9A-Fa-f]{4,8}|[0-9A-Fa-f]{4,8}h|[A-Z][A-Z0-9_x]{1,22})\b",
        )
        .expect("valid regex")
    });

    for tb in &indexes.tables {
        // Skip field description tables since they are already indexed in `registers`
        let is_reg_field_tb = tb.headers.len() >= 4
            && tb.headers[0].to_ascii_lowercase().contains("bit")
            && tb.headers[1].to_ascii_lowercase().contains("field");
        if is_reg_field_tb {
            continue;
        }

        let mut periph = tb
            .caption
            .as_deref()
            .map(infer_peripheral_name)
            .unwrap_or_else(|| "SYS".to_string());
        if periph == "SYS" || periph.eq_ignore_ascii_case("continued") {
            periph = indexes
                .sections
                .iter()
                .find(|sec| sec.file_rel_path == tb.section_file)
                .map(|sec| infer_peripheral_name(&sec.title))
                .unwrap_or_else(|| "SYS".to_string());
            if periph.eq_ignore_ascii_case("continued") {
                periph = "SYS".to_string();
            }
        }
        let caption_label = tb.caption.as_deref().unwrap_or(&tb.id);

        // Extract symbols from table caption
        for cap in re_hw.captures_iter(caption_label) {
            if let Some(m) = cap.get(1) {
                let sym = normalize_hw_token(m.as_str());
                if is_meaningful_hw_symbol(&sym) {
                    push_unique(XrefEntry {
                        symbol: sym,
                        kind: "table".to_string(),
                        peripheral: periph.clone(),
                        summary: format!(
                            "{} (cols: {})",
                            caption_label,
                            truncate_summary(&tb.headers.join(" | "), 70)
                        ),
                        location: format!("{}:{}", tb.section_file, tb.line_number),
                        page_number: tb.page_start,
                    });
                }
            }
        }

        let skip_pkg_cols = tb.headers.len() > 3;
        let pkg_col_flags: Vec<bool> = tb
            .headers
            .iter()
            .map(|h| skip_pkg_cols && is_package_pin_column_header(h))
            .collect();

        // Extract symbols from up to 16 columns of table rows, skipping pure package-pin coordinate columns
        for (row_idx, row) in tb.rows.iter().enumerate() {
            let row_line = tb.line_number + 2 + row_idx;
            let row_preview = truncate_summary(&row.join(" | "), 100);
            for (col_idx, cell) in row.iter().take(16).enumerate() {
                if pkg_col_flags.get(col_idx).copied().unwrap_or(false) {
                    continue;
                }
                for cap in re_hw.captures_iter(cell) {
                    if let Some(m) = cap.get(1) {
                        let sym = normalize_hw_token(m.as_str());
                        if (sym.len() >= 3 || sym.starts_with("0x"))
                            && is_meaningful_hw_symbol(&sym)
                        {
                            push_unique(XrefEntry {
                                symbol: sym,
                                kind: "table_row".to_string(),
                                peripheral: periph.clone(),
                                summary: format!("[{}] {}", caption_label, row_preview),
                                location: format!("{}:{}", tb.section_file, row_line),
                                page_number: tb.page_start,
                            });
                        }
                    }
                }
            }
        }
    }

    // 4. Index figures by hardware symbol in their caption
    for fig in &indexes.figures {
        let mut periph = infer_peripheral_name(&fig.caption);
        if periph == "SYS" || periph.eq_ignore_ascii_case("continued") {
            periph = indexes
                .sections
                .iter()
                .find(|sec| sec.file_rel_path == fig.section_file)
                .map(|sec| infer_peripheral_name(&sec.title))
                .unwrap_or_else(|| "SYS".to_string());
            if periph.eq_ignore_ascii_case("continued") {
                periph = "SYS".to_string();
            }
        }
        for cap in re_hw.captures_iter(&fig.caption) {
            if let Some(m) = cap.get(1) {
                let sym = normalize_hw_token(m.as_str());
                if is_meaningful_hw_symbol(&sym) {
                    push_unique(XrefEntry {
                        symbol: sym,
                        kind: "figure".to_string(),
                        peripheral: periph.clone(),
                        summary: format!(
                            "{} ({})",
                            fig.caption,
                            fig.image_rel_path.as_deref().unwrap_or("lazy")
                        ),
                        location: format!("{}:{}", fig.section_file, fig.line_number),
                        page_number: fig.page_number,
                    });
                }
            }
        }
    }

    entries
}

fn normalize_hw_token(raw: &str) -> String {
    if (raw.starts_with("0x") || raw.starts_with("0X")) && (raw.contains('.') || raw.contains('_'))
    {
        raw.chars().filter(|&c| c != '.' && c != '_').collect()
    } else {
        raw.to_string()
    }
}

fn is_package_pin_column_header(header: &str) -> bool {
    let h = header.trim().to_ascii_lowercase();
    if h.contains("name") || h.contains("signal") || h.contains("function") || h.contains("iomux") {
        return false;
    }
    const PKG_KEYWORDS: &[&str] = &[
        "lqfp", "ufqfpn", "wlcsp", "so8", "tssop", "vqfn", "sot", "vfc", "zcm", "bga",
    ];
    PKG_KEYWORDS.iter().any(|kw| h.contains(kw)) || h.ends_with(" pin") || h.ends_with("ball")
}

fn is_meaningful_hw_symbol(sym: &str) -> bool {
    if sym.len() < 2 || sym.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    let upper = sym.to_ascii_uppercase();
    const STOPWORDS: &[&str] = &[
        "AN",
        "AS",
        "AT",
        "BE",
        "BY",
        "DO",
        "GO",
        "IF",
        "IN",
        "IS",
        "IT",
        "NO",
        "OF",
        "ON",
        "OR",
        "SO",
        "TO",
        "UP",
        "US",
        "WE",
        "ALL",
        "AND",
        "ARE",
        "BIT",
        "CAN",
        "COL",
        "FOR",
        "HAS",
        "HEX",
        "LOW",
        "MAX",
        "MIN",
        "NOT",
        "OFF",
        "ONE",
        "OUT",
        "PIN",
        "ROW",
        "SEE",
        "SET",
        "THE",
        "TOP",
        "TWO",
        "TYP",
        "USE",
        "VIA",
        "YES",
        "BITS",
        "BYTE",
        "CODE",
        "DATA",
        "DESC",
        "EACH",
        "FROM",
        "HIGH",
        "MODE",
        "NAME",
        "NOTE",
        "NULL",
        "ONLY",
        "PAGE",
        "PORT",
        "READ",
        "SIZE",
        "TYPE",
        "UNIT",
        "USER",
        "WHEN",
        "WITH",
        "WORD",
        "ZERO",
        "FIELD",
        "GROUP",
        "LEVEL",
        "NOTES",
        "OTHER",
        "RESET",
        "START",
        "STATE",
        "TABLE",
        "VALUE",
        "WRITE",
        "ACCESS",
        "COLUMN",
        "DEVICE",
        "ENABLE",
        "FIGURE",
        "HEADER",
        "MEMORY",
        "NUMBER",
        "OFFSET",
        "OUTPUT",
        "REGION",
        "SIGNAL",
        "SOURCE",
        "STATUS",
        "SYSTEM",
        "TARGET",
        "VECTOR",
        "CONTINUED",
        "CONT",
        "REV",
        "DOC",
        "JANUARY",
        "FEBRUARY",
        "MARCH",
        "APRIL",
        "MAY",
        "JUNE",
        "JULY",
        "AUGUST",
        "SEPTEMBER",
        "OCTOBER",
        "NOVEMBER",
        "DECEMBER",
        "MONDAY",
        "TUESDAY",
        "WEDNESDAY",
        "THURSDAY",
        "FRIDAY",
        "SATURDAY",
        "SUNDAY",
        "SECTION",
        "CHAPTER",
        "DEFAULT",
        "TYPICAL",
        "MAXIMUM",
        "MINIMUM",
        "CONDITION",
        "CONDITIONS",
        "PARAMETER",
        "PARAMETERS",
        "DESCRIPTION",
        "FUNCTION",
        "FUNCTIONS",
        "SUMMARY",
        "ALTERNATE",
        "PACKAGE",
        "FOOTPRINT",
    ];
    !STOPWORDS.contains(&upper.as_str())
}

fn truncate_summary(s: &str, max_chars: usize) -> String {
    let clean = s.split_whitespace().collect::<Vec<_>>().join(" ");
    if clean.chars().count() <= max_chars {
        clean
    } else {
        let mut out: String = clean.chars().take(max_chars.saturating_sub(3)).collect();
        out.push_str("...");
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::{ExtractedTable, RegisterBitfield, SectionFile};

    #[test]
    fn test_build_xref_graph_extracts_registers_and_table_pins() {
        let mut indexes = BundleIndexes::default();
        indexes.sections.push(SectionFile {
            index: 1,
            section_id: "10".to_string(),
            title: "10 General-Purpose Input/Output (GPIO)".to_string(),
            level: 1,
            start_page: 557,
            end_page: 580,
            file_rel_path: "sections/10-gpio.md".to_string(),
            markdown: "# GPIO".to_string(),
            line_count: 20,
            table_count: 1,
            register_count: 1,
            figure_count: 0,
        });
        indexes.registers.push(RegisterBitfield {
            peripheral: "GPIO".to_string(),
            register: "FSUB_0".to_string(),
            offset: "400h".to_string(),
            reset_value: "00000000h".to_string(),
            bits: "3-0".to_string(),
            field_name: "CHANID".to_string(),
            access: "R/W".to_string(),
            field_reset: "0h".to_string(),
            description: "Channel ID".to_string(),
            section_file: "sections/10-gpio.md".to_string(),
            line_number: 148,
            page_number: 564,
        });
        indexes.tables.push(ExtractedTable {
            id: "table-4-1".to_string(),
            caption: Some("Table 4-1. Pin Attributes".to_string()),
            page_start: 12,
            page_end: 12,
            bbox: crate::model::BBox::new(0.0, 0.0, 100.0, 100.0),
            headers: vec!["Pin".to_string(), "Signal".to_string()],
            rows: vec![vec!["PA27".to_string(), "ADC0_IN0".to_string()]],
            section_file: "sections/04-pins.md".to_string(),
            line_number: 30,
        });

        let xrefs = build_xref_graph(&indexes);
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "GPIO" && x.kind == "section"));
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "FSUB_0" && x.kind == "register"));
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "CHANID" && x.kind == "bitfield"));
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "PA27" && x.kind == "table_row"));
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "ADC0_IN0" && x.kind == "table_row"));
    }

    #[test]
    fn test_pinout_table_extracts_beyond_col3_and_skips_package_cols() {
        let mut indexes = BundleIndexes::default();
        indexes.sections.push(SectionFile {
            index: 1,
            section_id: "4".to_string(),
            title: "4 Pinouts and Pin Description (IOMUX)".to_string(),
            level: 1,
            start_page: 20,
            end_page: 35,
            file_rel_path: "sections/04-pinout.md".to_string(),
            markdown: "# Pinouts".to_string(),
            line_count: 50,
            table_count: 1,
            register_count: 0,
            figure_count: 0,
        });
        indexes.tables.push(ExtractedTable {
            id: "table-12".to_string(),
            caption: Some("Table 12. Pin definition (continued)".to_string()),
            page_start: 25,
            page_end: 25,
            bbox: crate::model::BBox::new(0.0, 0.0, 100.0, 100.0),
            headers: vec![
                "LQFP48".to_string(),
                "UFQFPN32".to_string(),
                "WLCSP19".to_string(),
                "Pin name".to_string(),
                "Alternate functions".to_string(),
                "Memory base".to_string(),
            ],
            rows: vec![vec![
                "2".to_string(),
                "3".to_string(),
                "A1".to_string(),
                "PC14-OSC32_IN (PC14)".to_string(),
                "TIM1_BKIN2 / DD / KB".to_string(),
                "0x4002.1000".to_string(),
            ]],
            section_file: "sections/04-pinout.md".to_string(),
            line_number: 15,
        });

        let xrefs = build_xref_graph(&indexes);
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "PC14" && x.kind == "table_row" && x.peripheral == "IOMUX"));
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "TIM1_BKIN2" && x.kind == "table_row"));
        assert!(xrefs
            .iter()
            .any(|x| x.symbol == "0x40021000" && x.kind == "table_row"));
        assert!(!xrefs.iter().any(|x| x.symbol == "A1"));
        assert!(!xrefs.iter().any(|x| x.symbol == "DD"));
        assert!(!xrefs.iter().any(|x| x.symbol == "KB"));
    }
}
