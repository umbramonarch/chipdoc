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

/// Detects a register definition heading either from a single inline line (`Offset = ...`)
/// or from a numbered register heading (`6.4.1 GPIO port mode register (GPIOx_MODER)`)
/// followed within the next few lines by `Address offset: 0x...` and `Reset value: 0x...`.
pub fn parse_register_heading_with_lookahead(
    lines: &[TextLine],
    idx: usize,
) -> Option<ParsedRegisterHeading> {
    let line = &lines[idx];
    if let Some(inline) = parse_register_heading(line) {
        return Some(inline);
    }

    static RE_PAREN_REG_HEADING: OnceLock<Regex> = OnceLock::new();
    static RE_ADDR_OFFSET: OnceLock<Regex> = OnceLock::new();
    static RE_RESET_VAL: OnceLock<Regex> = OnceLock::new();

    let re_head = RE_PAREN_REG_HEADING.get_or_init(|| {
        Regex::new(r"^(\d+(?:\.\d+){1,5})\s+.+?\(\s*([A-Z][A-Za-z0-9_x]{2,28})\s*\)")
            .expect("valid regex")
    });
    let re_off = RE_ADDR_OFFSET.get_or_init(|| {
        Regex::new(r"(?i)^Address\s+offset\s*:\s*(0x[0-9A-Fa-f]+|[0-9A-Fa-f]+h)")
            .expect("valid regex")
    });
    let re_rst = RE_RESET_VAL.get_or_init(|| {
        Regex::new(r"(?i)^Reset\s+value\s*:\s*(0x[0-9A-Fa-fXx]+(?:\s+[0-9A-Fa-fXx]{2,4})*|[0-9A-Fa-fXx]+h)")
            .expect("valid regex")
    });

    let caps = re_head.captures(line.text.trim())?;
    let section_num = caps.get(1)?.as_str().to_string();
    let register_name = caps.get(2)?.as_str().to_string();

    let end = (idx + 22).min(lines.len());
    let mut found_offset: Option<String> = None;
    let mut found_reset: Option<String> = None;

    for next_line in &lines[idx + 1..end] {
        let t = next_line.text.trim();
        if re_head.is_match(t) {
            break;
        }
        if found_offset.is_none() {
            if let Some(oc) = re_off.captures(t) {
                found_offset = oc.get(1).map(|m| m.as_str().replace(' ', ""));
            }
        }
        if found_reset.is_none() {
            if let Some(rc) = re_rst.captures(t) {
                found_reset = rc.get(1).map(|m| m.as_str().replace(' ', ""));
            }
        }
        if found_offset.is_some() && found_reset.is_some() {
            break;
        }
    }

    let offset = found_offset?;
    let reset = found_reset.unwrap_or_else(|| "N/A".to_string());

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
            // Walk downward through the bit-box lines until we hit a "Table X-Y" caption, Field Descriptions table, or "Bits X:Y" prose block
            let mut j = i + 1;
            while j < lines.len() {
                let t = lines[j].text.trim();
                if t.starts_with("Table ")
                    || t.starts_with("Bit Field Type Reset")
                    || t.starts_with("Bits ")
                    || t.starts_with("Bit ")
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
                            || tok == "Res."
                            || matches!(
                                tok,
                                "rw" | "r" | "w" | "rc_w0" | "rc_w1" | "rs" | "rt_w" | "t" | " r"
                            )
                            || tok.chars().all(|c| {
                                c.is_ascii_uppercase()
                                    || c.is_ascii_digit()
                                    || matches!(c, '_' | '[' | ']' | ':' | '.')
                            })
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

/// Extracts a `RegisterBitfield` from a prose bitfield definition paragraph such as:
/// `Bits 31:0 MODE[15:0][1:0]: Port x configuration I/O pin y ...` or
/// `Bit 16 LCKK: Lock key ...`
#[allow(clippy::too_many_arguments)]
pub fn extract_bitfield_from_paragraph(
    text: &str,
    peripheral: &str,
    register_name: &str,
    offset: &str,
    reg_reset: &str,
    section_file: &str,
    line_number: usize,
    page_number: usize,
) -> Option<RegisterBitfield> {
    if register_name.is_empty() {
        return None;
    }
    static RE_PROSE_BITFIELD: OnceLock<Regex> = OnceLock::new();
    let re = RE_PROSE_BITFIELD.get_or_init(|| {
        Regex::new(
            r"^Bits?\s+(\d+(?:\s*[:\-]\s*\d+)?)\s+([A-Za-z0-9_]+(?:\[[A-Za-z0-9_:\-.\s]+\])*)\s*:\s*(.+)$",
        )
        .expect("valid regex")
    });

    let trimmed = text.trim();
    let caps = re.captures(trimmed)?;
    let bits = caps
        .get(1)?
        .as_str()
        .replace(':', "-")
        .split_whitespace()
        .collect::<String>();
    let field_name = caps.get(2)?.as_str().trim().to_string();
    let desc_raw = caps.get(3)?.as_str().trim();
    let description = desc_raw.split_whitespace().collect::<Vec<_>>().join(" ");

    let low = description.to_ascii_lowercase();
    let access = if low.contains("read-only") || low.contains("read only") {
        "RO"
    } else if low.contains("write-only") || low.contains("write only") {
        "WO"
    } else if low.contains("cleared by writing 1") || low.contains("clear by writing 1") {
        "RW1C"
    } else {
        "R/W"
    };

    Some(RegisterBitfield {
        peripheral: peripheral.to_string(),
        register: register_name.to_string(),
        offset: if offset.is_empty() {
            "N/A".to_string()
        } else {
            offset.to_string()
        },
        reset_value: if reg_reset.is_empty() {
            "N/A".to_string()
        } else {
            reg_reset.to_string()
        },
        bits,
        field_name,
        access: access.to_string(),
        field_reset: "-".to_string(),
        description,
        section_file: section_file.to_string(),
        line_number,
        page_number,
    })
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
    fn test_parse_register_heading_with_lookahead_and_prose_bitfield() {
        let lines = vec![
            TextLine {
                text: "6.4.1 GPIO port mode register (GPIOx_MODER)".to_string(),
                bbox: BBox::new(0.0, 10.0, 300.0, 22.0),
                font_size: 12.0,
                is_bold: true,
                spans: vec![],
            },
            TextLine {
                text: "Address offset: 0x00".to_string(),
                bbox: BBox::new(0.0, 25.0, 200.0, 35.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
            TextLine {
                text: "Reset value: 0xEBFF FFFF".to_string(),
                bbox: BBox::new(0.0, 38.0, 200.0, 48.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
        ];
        let parsed =
            parse_register_heading_with_lookahead(&lines, 0).expect("should parse lookahead reg");
        assert_eq!(parsed.section_num, "6.4.1");
        assert_eq!(parsed.register_name, "GPIOx_MODER");
        assert_eq!(parsed.offset, "0x00");
        assert_eq!(parsed.reset, "0xEBFFFFFF");

        let bf = extract_bitfield_from_paragraph(
            "Bits 31:0 MODE[15:0][1:0]: Port x configuration I/O pin y (y = 15 to 0) These bits are written by software.",
            "GPIO",
            "GPIOx_MODER",
            "0x00",
            "0xEBFFFFFF",
            "sections/06-gpio.md",
            310,
            182,
        )
        .expect("should parse prose bitfield");
        assert_eq!(bf.bits, "31-0");
        assert_eq!(bf.field_name, "MODE[15:0][1:0]");
        assert_eq!(bf.access, "R/W");
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
