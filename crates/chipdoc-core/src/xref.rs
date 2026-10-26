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
        let reg_loc = format!("{}:{}", bf.section_file, bf.line_number.saturating_sub(2));
        let reg_key = (
            bf.peripheral.clone(),
            bf.register.clone(),
            bf.section_file.clone(),
        );
        if seen_reg_headers.insert(reg_key) {
            push_unique(XrefEntry {
                symbol: bf.register.clone(),
                kind: "register".to_string(),
                peripheral: bf.peripheral.clone(),
                summary: format!(
                    "{}.{} (Offset = {}, Reset = {})",
                    bf.peripheral, bf.register, bf.offset, bf.reset_value
                ),
                location: reg_loc.clone(),
                page_number: bf.page_number,
            });

            if bf.peripheral != "SYS" && !bf.peripheral.is_empty() {
                push_unique(XrefEntry {
                    symbol: format!("{}.{}", bf.peripheral, bf.register),
                    kind: "register".to_string(),
                    peripheral: bf.peripheral.clone(),
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
                peripheral: bf.peripheral.clone(),
                summary: format!(
                    "{}.{}[{}] ({}, reset={}): {}",
                    bf.peripheral,
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
        Regex::new(r"\b([A-Z][A-Z0-9_]{1,18}|0x[0-9A-Fa-f]{4,8}|[0-9A-Fa-f]{4,8}h)\b")
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

        let periph = tb
            .caption
            .as_deref()
            .map(infer_peripheral_name)
            .unwrap_or_else(|| "SYS".to_string());
        let caption_label = tb.caption.as_deref().unwrap_or(&tb.id);

        // Extract symbols from table caption
        for cap in re_hw.captures_iter(caption_label) {
            if let Some(m) = cap.get(1) {
                let sym = m.as_str();
                if is_meaningful_hw_symbol(sym) {
                    push_unique(XrefEntry {
                        symbol: sym.to_string(),
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

        // Extract symbols from the first 3 columns of table rows (pins like PA0, peripherals, offsets, parameters)
        for (row_idx, row) in tb.rows.iter().enumerate() {
            let row_line = tb.line_number + 2 + row_idx;
            let row_preview = truncate_summary(&row.join(" | "), 100);
            for cell in row.iter().take(3) {
                for cap in re_hw.captures_iter(cell) {
                    if let Some(m) = cap.get(1) {
                        let sym = m.as_str();
                        if is_meaningful_hw_symbol(sym) {
                            push_unique(XrefEntry {
                                symbol: sym.to_string(),
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
        let periph = infer_peripheral_name(&fig.caption);
        for cap in re_hw.captures_iter(&fig.caption) {
            if let Some(m) = cap.get(1) {
                let sym = m.as_str();
                if is_meaningful_hw_symbol(sym) {
                    push_unique(XrefEntry {
                        symbol: sym.to_string(),
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

fn is_meaningful_hw_symbol(sym: &str) -> bool {
    if sym.len() < 2 || sym.chars().all(|c| c.is_ascii_digit()) {
        return false;
    }
    const STOPWORDS: &[&str] = &[
        "AN", "AS", "AT", "BE", "BY", "DO", "GO", "IF", "IN", "IS", "IT", "NO", "OF", "ON", "OR",
        "SO", "TO", "UP", "US", "WE", "ALL", "AND", "ARE", "BIT", "CAN", "COL", "FOR", "HAS",
        "HEX", "LOW", "MAX", "MIN", "NOT", "OFF", "ONE", "OUT", "PIN", "ROW", "SEE", "SET", "THE",
        "TOP", "TWO", "TYP", "USE", "VIA", "YES", "BITS", "BYTE", "CODE", "DATA", "DESC", "EACH",
        "FROM", "HIGH", "MODE", "NAME", "NOTE", "NULL", "ONLY", "PAGE", "PORT", "READ", "SIZE",
        "TYPE", "UNIT", "USER", "WHEN", "WITH", "WORD", "ZERO", "FIELD", "GROUP", "LEVEL", "NOTES",
        "OTHER", "RESET", "START", "STATE", "TABLE", "VALUE", "WRITE", "ACCESS", "COLUMN",
        "DEVICE", "ENABLE", "FIGURE", "HEADER", "MEMORY", "NUMBER", "OFFSET", "OUTPUT", "REGION",
        "SIGNAL", "SOURCE", "STATUS", "SYSTEM", "TARGET", "VECTOR",
    ];
    !STOPWORDS.contains(&sym)
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
}
