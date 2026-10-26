use crate::model::{BBox, ExtractedTable, RegisterBitfield, TextLine, TextSpan};
use regex::Regex;
use std::sync::OnceLock;

/// Information parsed from a register definition heading such as:
/// `10.3.1 FSUB_0 (Offset = 400h) [Reset = 00000000h]`
/// or `14.3.12 UART0_CTL0 Register (Offset = 0x1100) [Reset = 0x00000038]`
#[derive(Debug, Clone)]
pub struct ParsedRegisterHeading {
    pub section_num: String,
    pub register_name: String,
    pub offset: String,
    pub reset: String,
    pub bbox: BBox,
}

/// Detects whether a `TextLine` is a register definition heading and extracts its metadata.
pub fn parse_register_heading(line: &TextLine) -> Option<ParsedRegisterHeading> {
    static RE_REG_HEADING: OnceLock<Regex> = OnceLock::new();
    let re = RE_REG_HEADING.get_or_init(|| {
        Regex::new(
            r"^(\d+(?:\.\d+){1,5})\s+([A-Za-z0-9_]+)(?:\s+Register)?\s*\(\s*Offset\s*=\s*([0-9A-Fa-fxhXH]+)\s*\)\s*(?:\[\s*[Rr]eset\s*=\s*([0-9A-Fa-fxhXH]+)\s*\])?",
        )
        .expect("valid regex")
    });

    let caps = re.captures(line.text.trim())?;
    let section_num = caps.get(1)?.as_str().to_string();
    let register_name = caps.get(2)?.as_str().to_string();
    let offset = caps.get(3)?.as_str().to_string();
    let reset = caps
        .get(4)
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "N/A".to_string());

    Some(ParsedRegisterHeading {
        section_num,
        register_name,
        offset,
        reset,
        bbox: line.bbox,
    })
}

/// Detects and suppresses redundant visual 16-/32-bit register bit-box diagrams
/// (e.g. `Figure 10-4. FSUB_0` followed by `31 30 29 ... 16`, `RESERVED`, `R/W-0h`, `15 14 ... 0`)
/// by marking the spans inside the visual bit-box region as `consumed = true`.
/// Returns the number of visual bit-boxes suppressed on the page.
pub fn suppress_visual_register_bitboxes(
    spans: &mut [TextSpan],
    lines: &[TextLine],
    tables: &[ExtractedTable],
) -> usize {
    static RE_BIT_ROW: OnceLock<Regex> = OnceLock::new();
    static RE_ACCESS_ROW: OnceLock<Regex> = OnceLock::new();

    let re_bits = RE_BIT_ROW.get_or_init(|| {
        // Matches lines of consecutive bit numbers like "31 30 29 28 27 26 25 24..." or "15 14 13 12 ... 0"
        Regex::new(r"^(?:31\s+30\s+29\s+28|23\s+22\s+21\s+20|15\s+14\s+13\s+12|7\s+6\s+5\s+4)\b")
            .expect("valid regex")
    });
    let re_access = RE_ACCESS_ROW.get_or_init(|| {
        // Matches lines made of access-reset tokens like "R/W-0h", "R/W-", "WK-0h", "K-0h", "0h"
        Regex::new(
            r"^(?:(?:R/W|RO|WO|RW1C|W1C|WK|RC|RS|R|W|K)(?:-[0-9A-Fa-fXx]*h?)?|[0-9A-Fa-fXx]+h)\s*(?:(?:R/W|RO|WO|RW1C|W1C|WK|RC|RS|R|W|K)(?:-[0-9A-Fa-fXx]*h?)?|[0-9A-Fa-fXx]+h|\s)*$",
        )
        .expect("valid regex")
    });

    // Find vertical intervals on the page that contain visual bit-boxes
    let mut bitbox_regions: Vec<BBox> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if re_bits.is_match(lines[i].text.trim()) {
            let mut region = lines[i].bbox;
            // Include preceding "Figure X-Y. <REG>" caption if right above it
            if i > 0 {
                let prev = lines[i - 1].text.trim();
                if prev.starts_with("Figure ") && (lines[i].bbox.y0 - lines[i - 1].bbox.y1) < 35.0 {
                    region = region.union(&lines[i - 1].bbox);
                }
            }
            // Walk downward through the bit-box lines until we hit a "Table X-Y" caption or Field Descriptions table
            let mut j = i + 1;
            while j < lines.len() {
                let t = lines[j].text.trim();
                if t.starts_with("Table ")
                    || t.starts_with("Bit Field Type Reset")
                    || (lines[j].bbox.y0 - region.y1) > 95.0
                {
                    break;
                }
                // Also if there is a Field Descriptions table below this line on the same page,
                // everything between the initial `31 30 29 28` bit row and the `Table X-Y` caption is part of the visual bit-box.
                let above_field_table = tables.iter().any(|tb| {
                    is_field_description_table(tb)
                        && tb.bbox.y0 >= lines[j].bbox.y0 - 2.0
                        && (tb.bbox.y0 - region.y0) <= 260.0
                });
                let is_bit_line = above_field_table
                    || re_bits.is_match(t)
                    || re_access.is_match(t)
                    || t.starts_with("LEGEND:")
                    || t.split_whitespace().all(|tok| {
                        tok == "RESERVED"
                            || tok
                                .chars()
                                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                    });
                if is_bit_line {
                    region = region.union(&lines[j].bbox);
                    j += 1;
                } else {
                    break;
                }
            }
            bitbox_regions.push(region);
            i = j;
        } else {
            i += 1;
        }
    }

    if bitbox_regions.is_empty() {
        return 0;
    }

    // Ensure we don't accidentally suppress a real Field Descriptions table
    for reg_box in &bitbox_regions {
        for sp in spans.iter_mut() {
            if !sp.consumed && reg_box.contains_center_of(&sp.bbox, 2.5) {
                // Only suppress if it's not inside an ExtractedTable that is a Field Descriptions table
                let in_field_table = tables.iter().any(|tb| {
                    is_field_description_table(tb) && tb.bbox.contains_center_of(&sp.bbox, 1.0)
                });
                if !in_field_table {
                    sp.consumed = true;
                }
            }
        }
    }

    bitbox_regions.len()
}

/// Checks whether an extracted table is a Register Field Descriptions table
/// (`Bit | Field | Type | Reset | Description`).
pub fn is_field_description_table(table: &ExtractedTable) -> bool {
    if table.headers.len() < 4 {
        return false;
    }
    let h0 = table.headers[0].to_ascii_lowercase();
    let h1 = table.headers[1].to_ascii_lowercase();
    (h0.contains("bit") || h0.contains("bits")) && (h1.contains("field") || h1.contains("name"))
}

/// Builds a compact 1-line bit-map summary from a `Field Descriptions` table:
/// e.g. `> **Bit Map**: `[31:4] RESERVED (R/W, 0h)` | `[3:0] CHANID (R/W, 0h)``
pub fn build_bitmap_summary_from_table(table: &ExtractedTable) -> Option<String> {
    if !is_field_description_table(table) || table.rows.is_empty() {
        return None;
    }
    let mut parts = Vec::new();
    for row in &table.rows {
        if row.len() >= 4 {
            let bits = row[0].trim();
            let field = row[1].trim();
            let access = row[2].trim();
            let reset = row[3].trim();
            if !bits.is_empty() && !field.is_empty() {
                parts.push(format!("`[{}] {} ({}, {})`", bits, field, access, reset));
            }
        }
    }
    if parts.is_empty() {
        None
    } else {
        Some(format!("> **Bit Map**: {}", parts.join(" | ")))
    }
}

/// Extracts `RegisterBitfield` entries from a `Field Descriptions` table for `index/registers.tsv`.
pub fn extract_bitfields_from_table(
    table: &ExtractedTable,
    peripheral: &str,
    register_name: &str,
    offset: &str,
    reg_reset: &str,
    section_file: &str,
    line_number: usize,
) -> Vec<RegisterBitfield> {
    if !is_field_description_table(table) {
        return Vec::new();
    }
    let mut out = Vec::new();
    for (idx, row) in table.rows.iter().enumerate() {
        if row.len() < 4 {
            continue;
        }
        let bits = row[0].trim().to_string();
        let field_name = row[1].trim().to_string();
        let access = row[2].trim().to_string();
        let field_reset = row[3].trim().to_string();
        let description = row
            .get(4)
            .map(|d| {
                d.replace("<br>", "; ")
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .unwrap_or_default();

        if bits.is_empty() || field_name.is_empty() {
            continue;
        }

        out.push(RegisterBitfield {
            peripheral: peripheral.to_string(),
            register: register_name.to_string(),
            offset: offset.to_string(),
            reset_value: reg_reset.to_string(),
            bits,
            field_name,
            access,
            field_reset,
            description,
            section_file: section_file.to_string(),
            line_number: line_number + 2 + idx,
            page_number: table.page_start,
        });
    }
    out
}

/// Infers the peripheral acronym from a section title or register table caption
/// (e.g., `"10.3 GPIO Registers"` -> `"GPIO"`, `"2.7 SYSCTL_C1105_C1106 Registers"` -> `"SYSCTL"`).
pub fn infer_peripheral_name(section_title: &str) -> String {
    static RE_PAREN_ACRONYM: OnceLock<Regex> = OnceLock::new();
    static RE_REGS_PREFIX: OnceLock<Regex> = OnceLock::new();

    let re_paren = RE_PAREN_ACRONYM
        .get_or_init(|| Regex::new(r"\(([A-Za-z0-9_]{2,12})\)").expect("valid regex"));
    let re_regs = RE_REGS_PREFIX.get_or_init(|| {
        Regex::new(r"(?:^\d+(?:\.\d+)*\s+)?([A-Z][A-Za-z0-9_]{1,18})\s+Registers\b")
            .expect("valid regex")
    });

    if let Some(caps) = re_regs.captures(section_title.trim()) {
        if let Some(m) = caps.get(1) {
            return m.as_str().to_string();
        }
    }
    if let Some(caps) = re_paren.captures(section_title.trim()) {
        if let Some(m) = caps.get(1) {
            return m.as_str().to_string();
        }
    }

    // Fallback: first uppercase token of length >= 2 after leading section numbers
    for tok in section_title.split_whitespace() {
        let clean = tok.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_');
        if clean.len() >= 2
            && clean
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            && !clean.chars().all(|c| c.is_ascii_digit())
        {
            return clean.to_string();
        }
    }

    "SYS".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_register_heading() {
        let line = TextLine {
            text: "10.3.1 FSUB_0 (Offset = 400h) [Reset = 00000000h]".to_string(),
            bbox: BBox::new(0.0, 0.0, 400.0, 14.0),
            font_size: 12.0,
            is_bold: true,
            spans: vec![],
        };
        let parsed = parse_register_heading(&line).expect("should parse register heading");
        assert_eq!(parsed.section_num, "10.3.1");
        assert_eq!(parsed.register_name, "FSUB_0");
        assert_eq!(parsed.offset, "400h");
        assert_eq!(parsed.reset, "00000000h");
    }

    #[test]
    fn test_infer_peripheral_name() {
        assert_eq!(
            infer_peripheral_name("10 General-Purpose Input/Output (GPIO)"),
            "GPIO"
        );
        assert_eq!(infer_peripheral_name("14.3 UART Registers"), "UART");
        assert_eq!(
            infer_peripheral_name("2.7 SYSCTL_C1105_C1106 Registers"),
            "SYSCTL_C1105_C1106"
        );
    }
}
