use crate::model::{RawPageData, TextLine, TextSpan};
use rayon::prelude::*;
use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::sync::OnceLock;

/// Statistics collected while cleaning running headers, footers, and boilerplate.
#[derive(Debug, Default, Clone, Copy)]
pub struct CleanStats {
    pub header_footer_spans_stripped: usize,
}

/// Normalizes technical text to eliminate Unicode grep traps, inline markup tags, and subscript kerning collisions:
/// 1. Strips inline HTML/XML markup tags (`<Sup>`, `</Sup>`, `<sub>`, `<b>`, etc.).
/// 2. Unicode en-dash (`U+2013`), figure dash (`U+2012`), and minus sign (`U+2212`) before digits
///    or in ranges (`31–4`, `–0.3 V`, `–40°C`) -> ASCII `-`.
/// 3. Single-glyph units (`℃` `U+2103` -> `°C`, `℉` `U+2109` -> `°F`, `Ω` `U+2126` -> `Ω`,
///    `µ` `U+00B5` -> `u` or `µ` preserved cleanly, non-breaking spaces -> ASCII space).
/// 4. Subscript kerning collisions common in electrical spec tables (`VIInput voltage` -> `VI Input voltage`,
///    `TJOperating` -> `TJ Operating`).
pub fn normalize_technical_text(input: &str) -> String {
    static RE_INLINE_MARKUP: OnceLock<Regex> = OnceLock::new();
    static RE_NUMERIC_ENDASH: OnceLock<Regex> = OnceLock::new();
    static RE_SUBSCRIPT_COLLISION: OnceLock<Regex> = OnceLock::new();
    static RE_GLUED_XREF: OnceLock<Regex> = OnceLock::new();
    static RE_MULTI_SPACE: OnceLock<Regex> = OnceLock::new();

    let re_markup = RE_INLINE_MARKUP.get_or_init(|| {
        Regex::new(r"(?i)</?(?:sup|sub|b|i|u|em|strong|span|font|br)\b[^>]*>").expect("valid regex")
    });
    let re_endash = RE_NUMERIC_ENDASH.get_or_init(|| {
        // Match en-dash/minus before a digit (e.g. "–0.3", "–40") or between digits/hex ("31–0")
        Regex::new(r"([0-9A-Fa-f\s(\[=])[–−‒](\d)|^[–−‒](\d)").expect("valid regex")
    });
    let re_sub = RE_SUBSCRIPT_COLLISION.get_or_init(|| {
        // Matches 1-letter electrical symbol + subscript fused with a capitalized specification word:
        // e.g. "VIInput", "VOOutput", "TJOperating", "IIOLeakage", "VDDSupply"
        Regex::new(r"\b((?:V|I|T|R|C|f)(?:I[HLO]?|O[HLO]?|DD[A]?|SS[A]?|CC|BAT|REF[+-]?|J|A|STG|OSC|CLK|PU|PD))(Input|Output|Leakage|Operating|Supply|Voltage|Current|Thermal|Junction|Ambient|Storage|Clock|Frequency|Capacitance|Resistance|High|Low|Pull)\b")
            .expect("valid regex")
    });
    let re_xref = RE_GLUED_XREF.get_or_init(|| {
        Regex::new(r"\b((?:Figure|Fig\.|Table|Section)\s+\d+(?:[-\.]\d+)*)([a-z]{2,})\b")
            .expect("valid regex")
    });
    let re_space = RE_MULTI_SPACE.get_or_init(|| Regex::new(r"[ \t]{2,}").expect("valid regex"));

    // Fast character-level replacements first
    let mut s = String::with_capacity(input.len() + 8);
    for ch in input.chars() {
        match ch {
            '\u{2103}' => s.push_str("°C"),
            '\u{2109}' => s.push_str("°F"),
            '\u{2126}' => s.push('Ω'),
            '\u{00A0}' | '\u{2007}' | '\u{202F}' | '\u{2009}' => s.push(' '),
            '\u{200B}' | '\u{200C}' | '\u{200D}' | '\u{FEFF}' | '\u{00AD}' => {} // zero-width / soft hyphen
            '\u{0002}' | '\u{2212}' => s.push('-'), // PDFium soft-hyphen (0x02) and mathematical minus -> ASCII '-'
            c if c.is_control() && c != '\n' && c != '\t' => {}
            _ => s.push(ch),
        }
    }

    // Strip inline HTML/XML markup tags before whitespace collapsing
    let s = re_markup.replace_all(&s, "").into_owned();

    // Replace en-dash when used as a negative sign or numeric/bit range
    let s = re_endash
        .replace_all(&s, |caps: &regex::Captures| {
            if let Some(m3) = caps.get(3) {
                format!("-{}", m3.as_str())
            } else {
                format!(
                    "{}-{}",
                    caps.get(1).map(|m| m.as_str()).unwrap_or(""),
                    caps.get(2).map(|m| m.as_str()).unwrap_or("")
                )
            }
        })
        .into_owned();

    // Separate fused electrical symbol + Capitalized word (e.g., "VIInput" -> "VI Input")
    let s = re_sub.replace_all(&s, "$1 $2").into_owned();

    // Separate glued cross-reference numbers + lowercase words (e.g., "Figure 22-2and" -> "Figure 22-2 and")
    let s = re_xref.replace_all(&s, "$1 $2").into_owned();

    re_space.replace_all(s.trim(), " ").into_owned()
}

/// Cleans PDF metadata and bookmark titles by stripping inline markup, normalizing Unicode,
/// and trimming trailing `. .` dot-leader artifact sequences.
pub fn clean_metadata_text(input: &str) -> String {
    let single_line = input.replace(['\r', '\n', '\t'], " ");
    let mut out = normalize_technical_text(&single_line).replace(" ]", "]");
    while out.ends_with(". .") {
        out.truncate(out.len() - 2);
        out = out.trim().to_string();
    }
    while out.ends_with(" .") {
        out.truncate(out.len() - 2);
        out = out.trim().to_string();
    }
    out
}

/// Checks whether a line of text is repetitive vendor header/footer or navigation noise.
pub fn is_boilerplate_line(text: &str) -> bool {
    let t = text.trim();
    if t.is_empty() {
        return true;
    }
    let lower = t.to_ascii_lowercase();
    if lower == "www.ti.com"
        || lower == "(continued)"
        || lower == "submit document feedback"
        || lower == "return to the summary table."
        || lower == "return to the summary table"
        || lower.starts_with("copyright ©")
        || lower.starts_with("copyright (c)")
        || lower.contains("submit document feedback")
    {
        return true;
    }

    // Standalone document revision footer/header line like "DOCX123B – JULY 2025 – REVISED JANUARY 2026"
    // or "176/989 DOC1000 Rev 5" / "DS12991 Rev 6 9/100"
    static RE_DOC_REV: OnceLock<Regex> = OnceLock::new();
    let re_rev = RE_DOC_REV.get_or_init(|| {
        Regex::new(r"(?i)^(?:[A-Z]{4,8}[0-9A-Z]{2,5}\s*[–\-—]\s*(?:JANUARY|FEBRUARY|MARCH|APRIL|MAY|JUNE|JULY|AUGUST|SEPTEMBER|OCTOBER|NOVEMBER|DECEMBER)\s+\d{4}|(?:\d+\s*/\s*\d+\s+)?[A-Z]{2,8}\d{3,6}\s+Rev\s+\d+(?:\s+\d+\s*/\s*\d+)?$)").expect("valid regex")
    });
    if re_rev.is_match(t) {
        return true;
    }

    // Standalone TOC / mini-TOC dot-leader line (e.g. "20.1 IWDT . . . . . . . . 1259")
    static RE_TOC_DOT_LEADER: OnceLock<Regex> = OnceLock::new();
    let re_toc = RE_TOC_DOT_LEADER.get_or_init(|| {
        Regex::new(r"(?:\.\s*){6,}\s*\d+(?:\s*[-–]\s*\d+)?\s*$|^(?:\.\s*){8,}$")
            .expect("valid regex")
    });
    if re_toc.is_match(t) {
        return true;
    }

    false
}

fn is_margin_page_number(text: &str) -> bool {
    static RE_PAGE_NUM: OnceLock<Regex> = OnceLock::new();
    let re = RE_PAGE_NUM.get_or_init(|| {
        Regex::new(r"^(?:\d{1,5}|\d{1,5}\s*/\s*\d{1,5}|(?i)[ivxlcdm]{1,6})$").expect("valid regex")
    });
    re.is_match(text.trim())
}

fn is_candidate_anchor_token(tok: &str) -> bool {
    if tok.len() < 4 {
        return false;
    }
    let has_upper = tok.chars().any(|c| c.is_ascii_uppercase());
    let has_digit = tok.chars().any(|c| c.is_ascii_digit());
    let upper_count = tok.chars().filter(|c| c.is_ascii_uppercase()).count();
    (has_upper && has_digit) || upper_count >= 5
}

/// Strips running headers/footers and normalizes text across all extracted pages.
pub fn clean_pages(pages: &mut [RawPageData]) -> CleanStats {
    let mut stats = CleanStats::default();
    if pages.is_empty() {
        return stats;
    }

    // 1. Identify text strings and document-wide anchor tokens that repeat in the top 9.5% or bottom 10.8%
    let mut margin_freq: HashMap<String, usize> = HashMap::new();
    let mut anchor_token_freq: HashMap<String, usize> = HashMap::new();
    let num_pages = pages.len();
    let min_repeat_pages = if num_pages >= 6 { 3 } else { 2 };
    let anchor_threshold = (num_pages / 8).clamp(5, 25);

    for page in pages.iter() {
        let top_limit = page.height * 0.095;
        let bottom_limit = page.height * 0.892;
        let mut seen_on_page = HashSet::new();
        let mut seen_tokens_on_page = HashSet::new();
        for span in &page.spans {
            if span.bbox.y1 <= top_limit || span.bbox.y0 >= bottom_limit {
                let norm = normalize_header_footer_key(&span.text);
                if !norm.is_empty() {
                    seen_on_page.insert(norm);
                }
                for tok in span.text.split(|c: char| !c.is_ascii_alphanumeric()) {
                    if is_candidate_anchor_token(tok) {
                        seen_tokens_on_page.insert(tok.to_string());
                    }
                }
            }
        }
        for key in seen_on_page {
            *margin_freq.entry(key).or_insert(0) += 1;
        }
        for tok in seen_tokens_on_page {
            *anchor_token_freq.entry(tok).or_insert(0) += 1;
        }
    }

    let doc_anchors: HashSet<String> = if num_pages >= 5 {
        anchor_token_freq
            .into_iter()
            .filter_map(|(tok, count)| (count >= anchor_threshold).then_some(tok))
            .collect()
    } else {
        HashSet::new()
    };

    // 2. Filter and normalize spans on each page in parallel across Rayon worker threads
    let stripped: usize = pages
        .par_iter_mut()
        .map(|page| {
            let top_limit = page.height * 0.095;
            let bottom_limit = page.height * 0.892;
            let mut page_stripped = 0usize;
            let mut evicted_margin_ys: Vec<f32> = Vec::new();

            for span in &mut page.spans {
                span.text = normalize_technical_text(&span.text);
                if span.text.is_empty() {
                    span.consumed = true;
                    continue;
                }

                let in_margin = span.bbox.y1 <= top_limit || span.bbox.y0 >= bottom_limit;
                if in_margin {
                    let key = normalize_header_footer_key(&span.text);
                    let is_repeated = num_pages >= 3
                        && !key.is_empty()
                        && margin_freq.get(&key).copied().unwrap_or(0) >= min_repeat_pages;
                    let has_doc_anchor = !doc_anchors.is_empty()
                        && span
                            .text
                            .split(|c: char| !c.is_ascii_alphanumeric())
                            .any(|tok| doc_anchors.contains(tok));
                    let is_page_num = is_margin_page_number(&span.text);
                    let is_boiler = is_boilerplate_line(&span.text);

                    if is_repeated || has_doc_anchor || is_page_num || is_boiler {
                        if is_repeated
                            || has_doc_anchor
                            || is_boiler
                            || (is_page_num && span.font_size <= 11.5)
                        {
                            evicted_margin_ys.push(span.bbox.center_y());
                        }
                        span.consumed = true;
                        page_stripped += 1;
                        continue;
                    }
                }

                if is_boilerplate_line(&span.text) {
                    span.consumed = true;
                    page_stripped += 1;
                }
            }

            // Whole-line margin eviction: evict orphaned chapter-title halves of running header/footer lines
            if !evicted_margin_ys.is_empty() {
                for span in &mut page.spans {
                    if span.consumed {
                        continue;
                    }
                    let in_margin = span.bbox.y1 <= top_limit || span.bbox.y0 >= bottom_limit;
                    if in_margin
                        && (!span.is_bold || span.font_size <= 11.0)
                        && evicted_margin_ys
                            .iter()
                            .any(|&ey| (span.bbox.center_y() - ey).abs() <= 4.0)
                    {
                        span.consumed = true;
                        page_stripped += 1;
                    }
                }
            }

            page_stripped
        })
        .sum();

    stats.header_footer_spans_stripped = stripped;
    stats
}

fn normalize_header_footer_key(s: &str) -> String {
    // Strip digits and common header/footer separators, lowercase, and sort tokens alphabetically
    // so alternating odd/even running headers ("<DocID> <Chapter>" vs "<Chapter> <DocID>") match.
    let cleaned: String = s
        .chars()
        .filter(|c| !c.is_ascii_digit())
        .map(|c| {
            if matches!(c, '|' | '-' | '–' | '—' | '/') {
                ' '
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect();
    let mut tokens: Vec<&str> = cleaned.split_whitespace().collect();
    tokens.sort_unstable();
    tokens.join(" ")
}

/// Groups unconsumed `TextSpan`s into sorted horizontal `TextLine`s.
pub fn assemble_text_lines(spans: &[TextSpan], y_tolerance: f32) -> Vec<TextLine> {
    let mut active: Vec<&TextSpan> = spans
        .iter()
        .filter(|s| !s.consumed && !s.text.trim().is_empty())
        .collect();

    if active.is_empty() {
        return Vec::new();
    }

    // Sort top-to-bottom by center_y, then left-to-right by x0
    active.sort_by(|a, b| {
        a.bbox
            .center_y()
            .partial_cmp(&b.bbox.center_y())
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| {
                a.bbox
                    .x0
                    .partial_cmp(&b.bbox.x0)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
    });

    let mut lines: Vec<Vec<TextSpan>> = Vec::new();
    for span in active {
        let cy = span.bbox.center_y();
        if let Some(last_line) = lines.last_mut() {
            let line_cy =
                last_line.iter().map(|s| s.bbox.center_y()).sum::<f32>() / (last_line.len() as f32);
            let max_h = last_line
                .iter()
                .map(|s| s.bbox.height())
                .fold(span.bbox.height(), f32::max);
            let last_line_max_font = last_line
                .iter()
                .map(|s| s.font_size)
                .fold(0.0_f32, f32::max);
            let last_line_y0 = last_line
                .iter()
                .map(|s| s.bbox.y0)
                .fold(f32::INFINITY, f32::min);
            let last_line_y1 = last_line
                .iter()
                .map(|s| s.bbox.y1)
                .fold(f32::NEG_INFINITY, f32::max);
            let tol = y_tolerance.max(max_h * 0.45);
            let is_script_attach = span.font_size < last_line_max_font * 0.85
                && span.text.trim().len() <= 4
                && span.bbox.y1 >= last_line_y0 - 1.5
                && span.bbox.y0 <= last_line_y1 + 1.5;
            if (cy - line_cy).abs() <= tol || is_script_attach {
                last_line.push(span.clone());
                continue;
            }
        }
        lines.push(vec![span.clone()]);
    }

    // Merge small script-only lines (e.g. superscript "2 2" for I²C/I²S or "® ®" for Arm® Cortex®)
    // into an adjacent normal body line when every span vertically overlaps the host line's band.
    if lines.len() >= 2 {
        let mut merged_into: Vec<Option<usize>> = vec![None; lines.len()];
        for i in 0..lines.len() {
            let script_max_font = lines[i].iter().map(|s| s.font_size).fold(0.0_f32, f32::max);
            let total_len: usize = lines[i].iter().map(|s| s.text.trim().len()).sum();
            if script_max_font <= 7.8 && total_len <= 8 {
                let candidates = [i.checked_add(1), i.checked_sub(1)];
                for cand in candidates.into_iter().flatten() {
                    if cand >= lines.len() || merged_into[cand].is_some() {
                        continue;
                    }
                    let host_max_font = lines[cand]
                        .iter()
                        .map(|s| s.font_size)
                        .fold(0.0_f32, f32::max);
                    let host_y0 = lines[cand]
                        .iter()
                        .map(|s| s.bbox.y0)
                        .fold(f32::INFINITY, f32::min);
                    let host_y1 = lines[cand]
                        .iter()
                        .map(|s| s.bbox.y1)
                        .fold(f32::NEG_INFINITY, f32::max);
                    if host_max_font >= script_max_font * 1.18
                        && lines[i]
                            .iter()
                            .all(|s| s.bbox.y1 >= host_y0 - 2.0 && s.bbox.y0 <= host_y1 + 2.0)
                    {
                        merged_into[i] = Some(cand);
                        break;
                    }
                }
            }
        }
        for i in 0..lines.len() {
            if let Some(target) = merged_into[i] {
                let script_spans = std::mem::take(&mut lines[i]);
                lines[target].extend(script_spans);
            }
        }
        lines.retain(|l| !l.is_empty());
    }

    static RE_XREF_TAIL: OnceLock<Regex> = OnceLock::new();
    let re_xref_tail = RE_XREF_TAIL.get_or_init(|| {
        Regex::new(r"(?:Figure|Fig\.|Table|Section)\s+\d+(?:[-\.]\d+)*$").expect("valid regex")
    });

    let mut result = Vec::with_capacity(lines.len());
    for mut line_spans in lines {
        line_spans.sort_by(|a, b| {
            a.bbox
                .x0
                .partial_cmp(&b.bbox.x0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut bbox = line_spans[0].bbox;
        let max_font = line_spans
            .iter()
            .map(|s| s.font_size)
            .fold(line_spans[0].font_size, f32::max);
        let mut bold_chars = 0usize;
        let mut total_chars = 0usize;
        let mut assembled = String::new();

        for (idx, sp) in line_spans.iter().enumerate() {
            bbox = bbox.union(&sp.bbox);
            let char_len = sp.text.chars().count();
            total_chars += char_len;
            if sp.is_bold {
                bold_chars += char_len;
            }

            if idx > 0 {
                let prev = &line_spans[idx - 1];
                let gap = sp.bbox.x0 - prev.bbox.x1;
                let min_font = sp.font_size.min(prev.font_size);
                let space_thresh = (min_font * 0.06).max(0.45);
                let style_changed =
                    prev.is_bold != sp.is_bold || (prev.font_size - sp.font_size).abs() >= 0.8;
                let phrase_boundary = style_changed
                    && gap > -1.5
                    && prev
                        .text
                        .chars()
                        .last()
                        .is_some_and(|c| c.is_alphanumeric() || c == ')' || c == ']')
                    && sp
                        .text
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_alphanumeric() || c == '(' || c == '[');

                let sp_trim = sp.text.trim();
                let prev_trim = prev.text.trim();

                let sp_is_script =
                    sp.font_size < max_font * 0.85 && sp_trim.len() <= 3 && gap <= 2.5;
                let prev_is_script =
                    prev.font_size < max_font * 0.85 && prev_trim.len() <= 3 && gap <= 2.5;

                let suppress_script_space = ((sp_is_script || gap <= 2.0)
                    && (sp_trim == "®" || sp_trim == "™"))
                    || (sp_is_script
                        && sp_trim.chars().all(|c| c.is_ascii_digit())
                        && prev_trim.ends_with('I'))
                    || (prev_is_script
                        && gap <= 1.5
                        && ((prev_trim.chars().all(|c| c.is_ascii_digit())
                            && assembled.ends_with(&format!("I{prev_trim}"))
                            && sp_trim.starts_with(['C', 'S']))
                            || ((prev_trim == "®" || prev_trim == "™")
                                && sp_trim.starts_with('-'))));

                let subscript_word_boundary = prev.font_size < sp.font_size * 0.86
                    && prev.bbox.center_y() > sp.bbox.center_y() + 0.8
                    && sp_trim
                        .chars()
                        .next()
                        .is_some_and(|c| c.is_ascii_uppercase())
                    && sp_trim
                        .chars()
                        .nth(1)
                        .is_some_and(|c| c.is_ascii_lowercase());

                let acronym_word_boundary = prev_trim.len() >= 2
                    && prev_trim
                        .chars()
                        .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit())
                    && sp_trim.len() >= 2
                    && sp_trim.chars().all(|c| c.is_ascii_lowercase());

                let forced_boundary = gap >= -0.4
                    && (acronym_word_boundary
                        || (prev_trim
                            .chars()
                            .last()
                            .is_some_and(|c| matches!(c, ')' | ']' | '™' | '®'))
                            && sp_trim
                                .chars()
                                .next()
                                .is_some_and(|c| c.is_ascii_alphanumeric()))
                        || (re_xref_tail.is_match(assembled.trim_end())
                            && sp_trim
                                .chars()
                                .next()
                                .is_some_and(|c| c.is_ascii_lowercase())));

                if !suppress_script_space
                    && (gap > space_thresh
                        || phrase_boundary
                        || subscript_word_boundary
                        || forced_boundary)
                    && !assembled.ends_with(' ')
                    && !sp.text.starts_with(' ')
                    && !is_tight_punctuation(&sp.text)
                {
                    assembled.push(' ');
                }
            }
            assembled.push_str(&sp.text);
        }

        let text = normalize_technical_text(&assembled);
        if !text.is_empty() {
            result.push(TextLine {
                text,
                bbox,
                font_size: max_font,
                is_bold: total_chars > 0 && bold_chars * 2 >= total_chars,
                spans: line_spans,
            });
        }
    }

    result
}

fn is_tight_punctuation(s: &str) -> bool {
    matches!(
        s.chars().next(),
        Some(',' | '.' | ';' | ':' | ')' | ']' | '}')
    )
}

///Generates a URL/filename-safe slug from a section or figure title.
pub fn slugify(title: &str) -> String {
    let mut out = String::with_capacity(title.len());
    let mut prev_dash = false;
    for ch in title.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    while out.ends_with('-') {
        out.pop();
    }
    if out.len() > 64 {
        out.truncate(64);
        while out.ends_with('-') {
            out.pop();
        }
    }
    if out.is_empty() {
        "section".to_string()
    } else {
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::BBox;

    fn make_span(text: &str, x0: f32, y0: f32, x1: f32, y1: f32, font_size: f32) -> TextSpan {
        TextSpan {
            text: text.to_string(),
            bbox: BBox::new(x0, y0, x1, y1),
            font_size,
            is_bold: false,
            is_mono: false,
            consumed: false,
        }
    }

    #[test]
    fn test_normalize_technical_text_handles_endash_celsius_and_subscripts() {
        assert_eq!(normalize_technical_text("–0.3 4.1 V"), "-0.3 4.1 V");
        assert_eq!(
            normalize_technical_text("-40℃ ≤ Tj ≤ 130℃"),
            "-40°C ≤ Tj ≤ 130°C"
        );
        assert_eq!(
            normalize_technical_text("VIInput voltage Applied to any pin"),
            "VI Input voltage Applied to any pin"
        );
        assert_eq!(normalize_technical_text("31–4 RESERVED"), "31-4 RESERVED");
    }

    #[test]
    fn test_clean_metadata_text_and_inline_markup_stripping() {
        assert_eq!(
            clean_metadata_text(
                "Datasheet - Arm<Sup>® </Sup>Cortex<Sup>®</Sup>-M0+ 32-bit MCU . . . ."
            ),
            "Datasheet - Arm® Cortex®-M0+ 32-bit MCU"
        );
        assert_eq!(
            clean_metadata_text("1.2 System Overview. ."),
            "1.2 System Overview."
        );
    }

    #[test]
    fn test_hardware_acronym_preservation() {
        // Ensure hardware acronyms are never falsely split by RE_SUBSCRIPT_COLLISION
        assert_eq!(
            normalize_technical_text("TRM for details"),
            "TRM for details"
        );
        assert_eq!(normalize_technical_text("TXFIFO level"), "TXFIFO level");
        assert_eq!(normalize_technical_text("I2C register"), "I2C register");
        assert_eq!(normalize_technical_text("TRMfor"), "TRMfor");
        assert_eq!(normalize_technical_text("TXFIFOlevel"), "TXFIFOlevel");
        assert_eq!(normalize_technical_text("I2Cregister"), "I2Cregister");
        assert_eq!(normalize_technical_text("CRCframe"), "CRCframe");
        assert_eq!(normalize_technical_text("RCCregister"), "RCCregister");
        assert_eq!(normalize_technical_text("VDDmax"), "VDDmax");

        // Adjacent spans with tight kerning gap (>= 0.55pt) preserve space between acronym and word
        let spans = vec![
            make_span("TRM", 50.0, 100.0, 72.0, 110.0, 10.0),
            make_span("for", 72.7, 100.0, 85.0, 110.0, 10.0),
            make_span("TXFIFO", 90.0, 100.0, 125.0, 110.0, 10.0),
            make_span("level", 125.7, 100.0, 148.0, 110.0, 10.0),
            make_span("I2C", 154.0, 100.0, 170.0, 110.0, 10.0),
            make_span("register", 170.7, 100.0, 205.0, 110.0, 10.0),
        ];
        let lines = assemble_text_lines(&spans, 2.5);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "TRM for TXFIFO level I2C register");
    }

    #[test]
    fn test_superscript_line_attachment_and_glued_xref_spacing() {
        // Superscript "2" (I2C) and "®" (Arm®) sitting slightly higher in y than the 10pt body line,
        // plus glued cross-reference "Figure 2-1" + "and"
        let spans = vec![
            make_span("Arm", 40.0, 100.0, 58.0, 110.0, 10.0),
            make_span("®", 58.2, 97.5, 62.0, 102.5, 6.0),
            make_span("Cortex", 62.5, 100.0, 92.0, 110.0, 10.0),
            make_span("I", 98.0, 100.0, 101.0, 110.0, 10.0),
            make_span("2", 101.2, 97.5, 104.5, 102.5, 6.0),
            make_span("C", 104.7, 100.0, 112.0, 110.0, 10.0),
            make_span("see Figure 2-1", 118.0, 100.0, 180.0, 110.0, 10.0),
            make_span("and", 180.1, 100.0, 196.0, 110.0, 10.0),
        ];
        let lines = assemble_text_lines(&spans, 2.5);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "Arm® Cortex I2C see Figure 2-1 and");
        assert_eq!(
            normalize_technical_text("See Figure 2-1and Section 22.2.4for Table 29gives"),
            "See Figure 2-1 and Section 22.2.4 for Table 29 gives"
        );
    }

    #[test]
    fn test_clean_pages_alternating_headers_and_nm_page_numbers() {
        let make_page = |page_number: usize, header: &str, footer_page: &str| RawPageData {
            page_number,
            width: 595.0,
            height: 842.0,
            spans: vec![
                make_span(header, 50.0, 35.0, 500.0, 45.0, 9.0),
                make_span("Body content line", 50.0, 200.0, 300.0, 212.0, 10.0),
                make_span(footer_page, 500.0, 760.0, 545.0, 770.0, 9.0),
            ],
            grid_segments: Vec::new(),
            graphic_bboxes: Vec::new(),
            image_bboxes: Vec::new(),
        };

        let mut pages = vec![
            make_page(1, "DOC1000 Rev 5 | Reset and clock control (RCC)", "1/989"),
            make_page(2, "Reset and clock control (RCC) | DOC1000 Rev 5", "2/989"),
            make_page(3, "DOC1000 Rev 5 | Reset and clock control (RCC)", "3/989"),
        ];

        let stats = clean_pages(&mut pages);
        assert_eq!(stats.header_footer_spans_stripped, 6);
        for page in &pages {
            let active: Vec<&str> = page
                .spans
                .iter()
                .filter(|s| !s.consumed)
                .map(|s| s.text.as_str())
                .collect();
            assert_eq!(active, vec!["Body content line"]);
        }
    }

    #[test]
    fn test_boilerplate_detection() {
        assert!(is_boilerplate_line("www.ti.com"));
        assert!(is_boilerplate_line("Submit Document Feedback"));
        assert!(is_boilerplate_line("Return to the Summary Table."));
        assert!(is_boilerplate_line(
            "Copyright © 2026 Acme Semiconductors Incorporated"
        ));
        assert!(is_boilerplate_line(
            "DOCX123B – JULY 2025 – REVISED JANUARY 2026"
        ));
        assert!(is_boilerplate_line(
            "20.1 IWDT Clock Configuration . . . . . . . . . . . . . 1259"
        ));
        assert!(!is_boilerplate_line("10.3 GPIO Registers"));
    }

    #[test]
    fn test_slugify() {
        assert_eq!(
            slugify("10.3 General-Purpose Input/Output (GPIO)"),
            "10-3-general-purpose-input-output-gpio"
        );
    }
}
