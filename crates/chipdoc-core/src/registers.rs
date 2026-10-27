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
    pub lines_consumed: usize,
}

/// Detects whether a `TextLine` is a register definition heading and extracts its metadata.
pub fn parse_register_heading(line: &TextLine) -> Option<ParsedRegisterHeading> {
    static RE_REG_HEADING: OnceLock<Regex> = OnceLock::new();
    let re = RE_REG_HEADING.get_or_init(|| {
        Regex::new(
            r"^(\d+(?:\.\d+){1,5})\s+([A-Za-z0-9_]+(?:\[[A-Za-z0-9_]+\])?)(?:\s+Register)?\s*\(\s*Offset\s*=\s*([0-9A-Fa-fxhXH]+(?:\s*\+\s*formula)?)\s*\)\s*(?:\[\s*[Rr]eset\s*=\s*([0-9A-Fa-fxhXH]+)\s*\])?",
        )
        .expect("valid regex")
    });

    let caps = re.captures(line.text.trim())?;
    let section_num = caps.get(1)?.as_str().to_string();
    let register_name = caps.get(2)?.as_str().to_string();
    let offset = caps
        .get(3)?
        .as_str()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
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
        lines_consumed: 1,
    })
}

/// Detects a register definition heading either from a single inline line (`Offset = ...`)
/// or from a numbered/unnumbered register heading (`6.4.1 GPIO port mode register (GPIOx_MODER)`,
/// 2-line wrapped `(DMAMUX_CxCR)`, `USB control register (USB_CNTR)`, etc.)
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
    static RE_SEC_NUM_PREFIX: OnceLock<Regex> = OnceLock::new();
    static RE_WRAPPED_PAREN_REG: OnceLock<Regex> = OnceLock::new();
    static RE_UNNUM_PAREN_REG: OnceLock<Regex> = OnceLock::new();
    static RE_NUM_NO_PAREN_REG: OnceLock<Regex> = OnceLock::new();
    static RE_ADDR_OFFSET: OnceLock<Regex> = OnceLock::new();
    static RE_RESET_VAL: OnceLock<Regex> = OnceLock::new();

    let re_head = RE_PAREN_REG_HEADING.get_or_init(|| {
        Regex::new(
            r"^(\d+(?:\.\d+){1,5})\s+.+?\(\s*([A-Z][A-Za-z0-9_x]{1,28}(?:\s+\d{1,2})?)\s*\)(?:\s*\([^)]*\))?\s*$",
        )
        .expect("valid regex")
    });
    let re_sec_prefix = RE_SEC_NUM_PREFIX
        .get_or_init(|| Regex::new(r"^(\d+(?:\.\d+){1,5})\s+[A-Z]").expect("valid regex"));
    let re_wrapped_paren = RE_WRAPPED_PAREN_REG.get_or_init(|| {
        Regex::new(
            r"^(?:[A-Za-z0-9_/\-\s\[\]]{0,45}?)?\(\s*([A-Z][A-Za-z0-9_x]{2,28}(?:\s+\d{1,2})?)\s*\)\s*$",
        )
        .expect("valid regex")
    });
    let re_unnum_paren = RE_UNNUM_PAREN_REG.get_or_init(|| {
        Regex::new(
            r"^[A-Z][A-Za-z0-9_/\-\s]{3,80}\(\s*([A-Z][A-Za-z0-9_x]{2,28}(?:\s+\d{1,2})?)\s*\)\s*$",
        )
        .expect("valid regex")
    });
    let re_num_no_paren = RE_NUM_NO_PAREN_REG.get_or_init(|| {
        Regex::new(
            r"^(\d+(?:\.\d+){1,5})\s+([A-Z][A-Za-z0-9_/\-\s]{3,75}\b(?:register|option\s+bytes))\s*$",
        )
        .expect("valid regex")
    });
    let re_off = RE_ADDR_OFFSET.get_or_init(|| {
        Regex::new(
            r"(?i)^Address(?:\s+offset)?\s*:\s*(0x[0-9A-Fa-f]+|[0-9A-Fa-f]+h|n\s*\*\s*\d+(?:\s*\+\s*\d+)?)",
        )
        .expect("valid regex")
    });
    let re_rst = RE_RESET_VAL.get_or_init(|| {
        Regex::new(
            r"(?i)^(?:(?:System|Backup\s+domain|Power[-\s]on(?:\s+reset)?(?:\s*\(POR\))?|POR)\s+)?(?:Reset(?:\s+value)?|POR|Read\s+only)\s*[:=]\s*(0x[0-9A-Fa-fXx]+(?:\s+[0-9A-Fa-fXx]{2,4})*|0b[01Xx]+(?:\s+[01Xx]{4})*|[0-9A-Fa-fXx]+h)",
        )
        .expect("valid regex")
    });

    let trimmed = line.text.trim();
    let (section_num, register_name, lines_consumed, bbox, max_offset_lookahead) =
        if let Some(caps) = re_head.captures(trimmed) {
            let sec = caps.get(1)?.as_str().to_string();
            let reg = caps.get(2)?.as_str().replace(' ', "");
            (sec, reg, 1usize, line.bbox, 22usize)
        } else if let Some(sec_caps) = re_sec_prefix.captures(trimmed) {
            let mut matched_wrapped = None;
            if idx + 1 < lines.len() {
                let next_line = &lines[idx + 1];
                let v_gap = next_line.bbox.y0 - line.bbox.y1;
                if v_gap <= 16.0 {
                    if let Some(wcaps) = re_wrapped_paren.captures(next_line.text.trim()) {
                        let sec = sec_caps.get(1)?.as_str().to_string();
                        let reg = wcaps.get(1)?.as_str().replace(' ', "");
                        matched_wrapped =
                            Some((sec, reg, 2usize, line.bbox.union(&next_line.bbox), 22usize));
                    }
                }
            }
            if let Some(wrapped) = matched_wrapped {
                wrapped
            } else if line.is_bold || line.font_size >= 10.5 {
                let ncaps = re_num_no_paren.captures(trimmed)?;
                let sec = ncaps.get(1)?.as_str().to_string();
                let title_part = ncaps.get(2)?.as_str();
                let reg = synthesize_register_id_from_title(title_part);
                if reg.is_empty() {
                    return None;
                }
                (sec, reg, 1usize, line.bbox, 5usize)
            } else {
                return None;
            }
        } else if line.is_bold || line.font_size >= 10.5 {
            let ucaps = re_unnum_paren.captures(trimmed)?;
            let reg = ucaps.get(1)?.as_str().replace(' ', "");
            if !reg.contains('_') {
                return None;
            }
            (String::new(), reg, 1usize, line.bbox, 6usize)
        } else {
            return None;
        };

    let start_look = idx + lines_consumed;
    let end = (start_look + 22).min(lines.len());
    let mut found_offset: Option<String> = None;
    let mut found_reset: Option<String> = None;

    for (step, next_line) in lines[start_look..end].iter().enumerate() {
        if found_offset.is_none() && step >= max_offset_lookahead {
            break;
        }
        let t = next_line.text.trim();
        if re_head.is_match(t)
            || re_sec_prefix.is_match(t)
            || ((next_line.is_bold || next_line.font_size >= 10.5) && re_unnum_paren.is_match(t))
        {
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
        bbox,
        lines_consumed,
    })
}

fn synthesize_register_id_from_title(title: &str) -> String {
    let stop_words = [
        "register", "data", "and", "option", "bytes", "the", "of", "for",
    ];
    let words: Vec<String> = title
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| !w.is_empty())
        .filter(|w| !stop_words.iter().any(|s| w.eq_ignore_ascii_case(s)))
        .map(|w| w.to_ascii_uppercase())
        .collect();
    if !words.is_empty() {
        words.join("_")
    } else {
        title
            .split(|c: char| !c.is_ascii_alphanumeric())
            .filter(|w| !w.is_empty())
            .map(|w| w.to_ascii_uppercase())
            .collect::<Vec<_>>()
            .join("_")
    }
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
    static RE_CONCAT_ACCESS: OnceLock<Regex> = OnceLock::new();
    static RE_FOOTNOTE_TOK: OnceLock<Regex> = OnceLock::new();

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
    let re_concat_access = RE_CONCAT_ACCESS
        .get_or_init(|| Regex::new(r"^(?:rw|rc_w0|rc_w1|rt_w|rs|r|w|t)+$").expect("valid regex"));
    let re_footnote_tok =
        RE_FOOTNOTE_TOK.get_or_init(|| Regex::new(r"^\(\d+\)$").expect("valid regex"));

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
                            || re_concat_access.is_match(tok)
                            || re_footnote_tok.is_match(tok)
                            || (tok
                                .chars()
                                .any(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                                && tok.chars().all(|c| {
                                    c.is_ascii_alphanumeric()
                                        || matches!(c, '_' | '[' | ']' | ':' | '.' | '+' | '-')
                                }))
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
            r"^Bits?\s+(\d+(?:\s*[:\-]\s*\d+)?(?:\s*,\s*\d+(?:\s*[:\-]\s*\d+)?)*)\s+([A-Za-z0-9_+]+(?:\[[A-Za-z0-9_:\-.\s]+\])*)(?:\s*:\s*|\s+)(.+)$",
        )
        .expect("valid regex")
    });

    let trimmed = text.trim();
    let caps = re.captures(trimmed)?;
    let m_field = caps.get(2)?;
    let m_desc = caps.get(3)?;
    let sep = &trimmed[m_field.end()..m_desc.start()];
    let has_colon = sep.contains(':');

    let bits = caps
        .get(1)?
        .as_str()
        .replace(':', "-")
        .split_whitespace()
        .collect::<String>();
    let field_name = m_field.as_str().trim().to_string();
    if !has_colon && (field_name.len() < 2 || !field_name.chars().any(|c| c.is_ascii_uppercase())) {
        return None;
    }

    let desc_raw = m_desc.as_str().trim();
    let description = desc_raw.split_whitespace().collect::<Vec<_>>().join(" ");

    let low = description.to_ascii_lowercase();
    let is_software_writable = low.contains("set and cleared by software")
        || low.contains("written by software")
        || low.contains("configured as output");
    let is_unconditional_ro = (low.contains("read-only") || low.contains("read only"))
        && !low.contains("not read-only")
        && !low.contains("not read only")
        && !low.contains("read-only when")
        && !low.contains("read only when")
        && !is_software_writable;
    let access = if is_unconditional_ro {
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

/// Infers a peripheral acronym specifically from a `<PERIPH> registers` heading
/// (e.g., `"14.3 UART Registers"` -> `Some("UART")`, `"16.4 TIM3/TIM4 registers"` -> `Some("TIM3")`).
pub fn infer_peripheral_from_registers_heading(section_title: &str) -> Option<String> {
    static RE_REGS_HEADING: OnceLock<Regex> = OnceLock::new();
    let re = RE_REGS_HEADING.get_or_init(|| {
        Regex::new(
            r"(?i)(?:^\d+(?:\.\d+)*\s+)?\b([A-Z][A-Z0-9_x]{1,18})(?:/[A-Z][A-Z0-9_x]{1,18})*\s+registers\b",
        )
        .expect("valid regex")
    });

    let excluded = [
        "BLOCK",
        "MANAGEMENT",
        "CONTROL",
        "SYSTEM",
        "PERIPHERAL",
        "SUMMARY",
        "GENERAL",
        "INTERRUPT",
        "OPTION",
    ];

    for caps in re.captures_iter(section_title.trim()) {
        if let Some(m) = caps.get(1) {
            let s = m.as_str();
            let upper_count = s.chars().filter(|c| c.is_ascii_uppercase()).count();
            let valid_chars = s
                .chars()
                .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' || c == 'x');
            if upper_count >= 2 && valid_chars {
                let up = s.to_ascii_uppercase();
                if !excluded.contains(&up.as_str()) {
                    return Some(up);
                }
            }
        }
    }
    None
}

/// Infers a peripheral prefix from an explicit `<PERIPH>_<REG>` register name
/// (e.g., `"TIM3_AF1"` -> `Some("TIM3")`, `"USB_CNTR"` -> `Some("USB")`, `"GPIOx_MODER"` -> `Some("GPIO")`).
pub fn infer_peripheral_from_register_name(register_name: &str) -> Option<String> {
    let (prefix, _rest) = register_name.split_once('_')?;
    let stripped = prefix.trim_end_matches('x');
    if stripped.len() >= 2
        && stripped
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
        && stripped.chars().any(|c| c.is_ascii_uppercase())
    {
        Some(stripped.to_string())
    } else {
        None
    }
}

/// Infers the peripheral acronym from a section title or register table caption
/// (e.g., `"10.3 GPIO Registers"` -> `"GPIO"`, `"2.7 SYSCTL_C1105_C1106 Registers"` -> `"SYSCTL"`).
pub fn infer_peripheral_name(section_title: &str) -> String {
    static RE_TRAILING_CONT: OnceLock<Regex> = OnceLock::new();
    static RE_PAREN_ACRONYM: OnceLock<Regex> = OnceLock::new();

    let re_cont = RE_TRAILING_CONT
        .get_or_init(|| Regex::new(r"(?i)\s*\(continued\)\s*$").expect("valid regex"));
    let re_paren = RE_PAREN_ACRONYM.get_or_init(|| {
        Regex::new(r"\(([A-Z][A-Z0-9_x]{1,15}(?:/[A-Z][A-Z0-9_x]{1,15})*)\)").expect("valid regex")
    });

    let clean_cow = re_cont.replace(section_title.trim(), "");
    let clean_title = clean_cow.trim();

    if let Some(periph) = infer_peripheral_from_registers_heading(clean_title) {
        return periph;
    }

    let skip_paren = [
        "CPU", "MCU", "SOC", "RAM", "ROM", "LSB", "MSB", "MAX", "MIN", "TYP", "PT",
    ];
    for caps in re_paren.captures_iter(clean_title) {
        if let Some(m) = caps.get(1) {
            if let Some(first) = m.as_str().split('/').next() {
                let upper_count = first.chars().filter(|c| c.is_ascii_uppercase()).count();
                let valid_chars = first
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_' || c == 'x');
                if upper_count >= 2 && valid_chars {
                    let up = first.to_ascii_uppercase();
                    if !skip_paren.contains(&up.as_str()) {
                        return up;
                    }
                }
            }
        }
    }

    // Fallback: first uppercase token of length >= 2 after leading section numbers
    let trimmed_start = clean_title.trim_start();
    if !trimmed_start.starts_with("Table ") && !trimmed_start.starts_with("Figure ") {
        let skip_words = [
            "THE",
            "AND",
            "FOR",
            "WITH",
            "FROM",
            "INTO",
            "OVERVIEW",
            "INTRODUCTION",
            "SUMMARY",
            "CHAPTER",
            "SECTION",
            "TABLE",
            "FIGURE",
        ];
        let words: Vec<&str> = clean_title.split_whitespace().collect();
        for (idx, &tok) in words.iter().enumerate() {
            let clean = tok.trim_matches(|c: char| !c.is_ascii_alphanumeric() && c != '_');
            if clean.len() >= 2
                && clean
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
                && !clean.chars().all(|c| c.is_ascii_digit())
            {
                if skip_words.contains(&clean) {
                    continue;
                }
                let next_word = words
                    .get(idx + 1)
                    .map(|w| w.trim_matches(|c: char| !c.is_ascii_alphanumeric()))
                    .unwrap_or("");
                if (clean == "DMA" && next_word.eq_ignore_ascii_case("burst"))
                    || (clean == "UIF" && next_word.eq_ignore_ascii_case("bit"))
                {
                    continue;
                }
                return clean.to_string();
            }
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
        assert_eq!(parsed.lines_consumed, 1);

        let arr_line = TextLine {
            text: "1.4.2.5 PWDMASSERASE[y] (Offset = 41C00024h + formula) [Reset = XXXXXXXXh]"
                .to_string(),
            bbox: BBox::new(0.0, 0.0, 400.0, 14.0),
            font_size: 12.0,
            is_bold: true,
            spans: vec![],
        };
        let arr_parsed =
            parse_register_heading(&arr_line).expect("should parse arrayed register heading");
        assert_eq!(arr_parsed.section_num, "1.4.2.5");
        assert_eq!(arr_parsed.register_name, "PWDMASSERASE[y]");
        assert_eq!(arr_parsed.offset, "41C00024h + formula");
        assert_eq!(arr_parsed.reset, "XXXXXXXXh");
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
        assert_eq!(parsed.lines_consumed, 1);

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
    fn test_lookahead_kerning_wrapped_and_unnumbered_headings() {
        // 1) Kerning space in acronym: (SYSCFG_ITLINE1 0) + Backup domain reset value
        let kern_lines = vec![
            TextLine {
                text: "9.3.18 SYSCFG interrupt line 10 status register (SYSCFG_ITLINE1 0)"
                    .to_string(),
                bbox: BBox::new(0.0, 10.0, 360.0, 22.0),
                font_size: 12.0,
                is_bold: true,
                spans: vec![],
            },
            TextLine {
                text: "Address offset: 0xA8".to_string(),
                bbox: BBox::new(0.0, 25.0, 200.0, 35.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
            TextLine {
                text: "Backup domain reset value: 0x0000 0000".to_string(),
                bbox: BBox::new(0.0, 38.0, 200.0, 48.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
        ];
        let p_kern = parse_register_heading_with_lookahead(&kern_lines, 0)
            .expect("should parse kerning register heading");
        assert_eq!(p_kern.register_name, "SYSCFG_ITLINE10");
        assert_eq!(p_kern.offset, "0xA8");
        assert_eq!(p_kern.reset, "0x00000000");

        // 2) 2-line wrapped register heading: (DMAMUX_CxCR) + binary reset value
        let wrapped_lines = vec![
            TextLine {
                text: "11.4.1 DMAMUX request line multiplexer channel x configuration register"
                    .to_string(),
                bbox: BBox::new(0.0, 10.0, 420.0, 22.0),
                font_size: 12.0,
                is_bold: true,
                spans: vec![],
            },
            TextLine {
                text: "(DMAMUX_CxCR)".to_string(),
                bbox: BBox::new(0.0, 24.0, 150.0, 36.0),
                font_size: 12.0,
                is_bold: true,
                spans: vec![],
            },
            TextLine {
                text: "Address offset: 0x00 + 0x04 * x".to_string(),
                bbox: BBox::new(0.0, 40.0, 200.0, 50.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
            TextLine {
                text: "Reset value: 0b1111 0000".to_string(),
                bbox: BBox::new(0.0, 52.0, 200.0, 62.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
        ];
        let p_wrap = parse_register_heading_with_lookahead(&wrapped_lines, 0)
            .expect("should parse 2-line wrapped register heading");
        assert_eq!(p_wrap.section_num, "11.4.1");
        assert_eq!(p_wrap.register_name, "DMAMUX_CxCR");
        assert_eq!(p_wrap.offset, "0x00");
        assert_eq!(p_wrap.reset, "0b11110000");
        assert_eq!(p_wrap.lines_consumed, 2);

        // 3) Unnumbered bold register heading: USB control register (USB_CNTR)
        let unnum_lines = vec![
            TextLine {
                text: "USB control register (USB_CNTR)".to_string(),
                bbox: BBox::new(0.0, 10.0, 260.0, 22.0),
                font_size: 11.5,
                is_bold: true,
                spans: vec![],
            },
            TextLine {
                text: "Address offset: 0x40".to_string(),
                bbox: BBox::new(0.0, 25.0, 200.0, 35.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
            TextLine {
                text: "Reset value: 0x0003".to_string(),
                bbox: BBox::new(0.0, 38.0, 200.0, 48.0),
                font_size: 10.0,
                is_bold: false,
                spans: vec![],
            },
        ];
        let p_unnum = parse_register_heading_with_lookahead(&unnum_lines, 0)
            .expect("should parse unnumbered bold register heading");
        assert_eq!(p_unnum.section_num, "");
        assert_eq!(p_unnum.register_name, "USB_CNTR");
        assert_eq!(p_unnum.offset, "0x40");
        assert_eq!(p_unnum.reset, "0x0003");
    }

    #[test]
    fn test_prose_bitfield_edge_cases() {
        // Non-contiguous bitfield
        let bf_nc = extract_bitfield_from_paragraph(
            "Bits 24, 14:12 OC2M[3:0]: Output compare 2 mode",
            "TIM3",
            "TIMx_CCMR1",
            "0x18",
            "0x00000000",
            "sections/16-tim.md",
            100,
            400,
        )
        .expect("should parse non-contiguous bitfield");
        assert_eq!(bf_nc.bits, "24,14-12");
        assert_eq!(bf_nc.field_name, "OC2M[3:0]");

        // Missing-colon bitfield with uppercase identifier
        let bf_nocol = extract_bitfield_from_paragraph(
            "Bits 31:16 REV_ID[15:0] Revision identifier",
            "DBG",
            "DBG_IDCODE",
            "0x00",
            "0x10000466",
            "sections/31-dbg.md",
            50,
            900,
        )
        .expect("should parse missing-colon bitfield");
        assert_eq!(bf_nocol.bits, "31-16");
        assert_eq!(bf_nocol.field_name, "REV_ID[15:0]");
        assert_eq!(bf_nocol.description, "Revision identifier");

        // Ordinary prose starting with "Bit 0 is ..." must NOT be parsed as a bitfield
        let prose_false = extract_bitfield_from_paragraph(
            "Bit 0 is set when the counter reaches the auto-reload value.",
            "TIM3",
            "TIMx_SR",
            "0x10",
            "0x0000",
            "sections/16-tim.md",
            120,
            401,
        );
        assert!(prose_false.is_none());

        // Conditional "read-only when" prose must remain R/W
        let bf_cond_ro = extract_bitfield_from_paragraph(
            "Bits 7:0 CC1S[1:0]: Capture/Compare 1 selection. This bit-field is read-only when CC1E = 1.",
            "TIM3",
            "TIMx_CCMR1",
            "0x18",
            "0x00000000",
            "sections/16-tim.md",
            140,
            402,
        )
        .expect("should parse conditional read-only bitfield");
        assert_eq!(bf_cond_ro.access, "R/W");
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
        assert_eq!(
            infer_peripheral_name("16 General-purpose timers (TIM3/TIM4)"),
            "TIM3"
        );
        assert_eq!(infer_peripheral_name("16.3.20 DMA burst mode"), "SYS");
        assert_eq!(infer_peripheral_name("17.3.3 UIF bit remapping"), "SYS");
    }
}
