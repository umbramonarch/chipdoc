use crate::model::{RawPageData, TextLine, TextSpan};
use rayon::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use std::sync::OnceLock;

/// Statistics collected while cleaning running headers, footers, and boilerplate.
#[derive(Debug, Default, Clone, Copy)]
pub struct CleanStats {
    pub header_footer_spans_stripped: usize,
}

/// Normalizes technical text to eliminate Unicode grep traps and subscript kerning collisions:
/// 1. Unicode en-dash (`U+2013`), figure dash (`U+2012`), and minus sign (`U+2212`) before digits
///    or in ranges (`31–4`, `–0.3 V`, `–40°C`) -> ASCII `-`.
/// 2. Single-glyph units (`℃` `U+2103` -> `°C`, `℉` `U+2109` -> `°F`, `Ω` `U+2126` -> `Ω`,
///    `µ` `U+00B5` -> `u` or `µ` preserved cleanly, non-breaking spaces -> ASCII space).
/// 3. Subscript kerning collisions common in electrical spec tables (`VIInput voltage` -> `VI Input voltage`,
///    `TJOperating` -> `TJ Operating`).
pub fn normalize_technical_text(input: &str) -> String {
    static RE_NUMERIC_ENDASH: OnceLock<Regex> = OnceLock::new();
    static RE_SUBSCRIPT_COLLISION: OnceLock<Regex> = OnceLock::new();
    static RE_MULTI_SPACE: OnceLock<Regex> = OnceLock::new();

    let re_endash = RE_NUMERIC_ENDASH.get_or_init(|| {
        // Match en-dash/minus before a digit (e.g. "–0.3", "–40") or between digits/hex ("31–0")
        Regex::new(r"([0-9A-Fa-f\s(\[=])[–−‒](\d)|^[–−‒](\d)").expect("valid regex")
    });
    let re_sub = RE_SUBSCRIPT_COLLISION.get_or_init(|| {
        // Matches common electrical parameter symbols fused with a capitalized English word:
        // e.g. "VIInput", "VOOutput", "TJOperating", "IIOLeakage", "VDDDSupply"
        Regex::new(r"\b([VIJTRCf][A-Z0-9]{1,5})([A-Z][a-z]{2,})").expect("valid regex")
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

    re_space.replace_all(s.trim(), " ").into_owned()
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

    // Standalone document revision footer/header line like "SLASFJ6B – JULY 2025 – REVISED JANUARY 2026"
    // or "176/989 RM0454 Rev 5" / "DS12991 Rev 6 9/100"
    static RE_DOC_REV: OnceLock<Regex> = OnceLock::new();
    let re_rev = RE_DOC_REV.get_or_init(|| {
        Regex::new(r"(?i)^(?:[A-Z]{4,8}[0-9A-Z]{2,5}\s*[–\-—]\s*(?:JANUARY|FEBRUARY|MARCH|APRIL|MAY|JUNE|JULY|AUGUST|SEPTEMBER|OCTOBER|NOVEMBER|DECEMBER)\s+\d{4}|(?:\d+\s*/\s*\d+\s+)?[A-Z]{2,8}\d{3,6}\s+Rev\s+\d+(?:\s+\d+\s*/\s*\d+)?$)").expect("valid regex")
    });
    if re_rev.is_match(t) {
        return true;
    }

    false
}

/// Strips running headers/footers and normalizes text across all extracted pages.
pub fn clean_pages(pages: &mut [RawPageData]) -> CleanStats {
    let mut stats = CleanStats::default();
    if pages.is_empty() {
        return stats;
    }

    // 1. Identify text strings that repeat in the top 7.5% or bottom 8.5% of >= 3 pages
    let mut margin_freq: HashMap<String, usize> = HashMap::new();
    let num_pages = pages.len();
    let min_repeat_pages = if num_pages >= 6 { 3 } else { 2 };

    for page in pages.iter() {
        let top_limit = page.height * 0.088;
        let bottom_limit = page.height * 0.912;
        let mut seen_on_page = std::collections::HashSet::new();
        for span in &page.spans {
            if span.bbox.y1 <= top_limit || span.bbox.y0 >= bottom_limit {
                let norm = normalize_header_footer_key(&span.text);
                if !norm.is_empty() {
                    seen_on_page.insert(norm);
                }
            }
        }
        for key in seen_on_page {
            *margin_freq.entry(key).or_insert(0) += 1;
        }
    }

    // 2. Filter and normalize spans on each page in parallel across Rayon worker threads
    let stripped: usize = pages
        .par_iter_mut()
        .map(|page| {
            let top_limit = page.height * 0.088;
            let bottom_limit = page.height * 0.912;
            let mut page_stripped = 0usize;

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
                        && margin_freq.get(&key).copied().unwrap_or(0) >= min_repeat_pages;
                    let is_page_num = span.text.chars().all(|c| c.is_ascii_digit());
                    if is_repeated || is_page_num || is_boilerplate_line(&span.text) {
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
            page_stripped
        })
        .sum();

    stats.header_footer_spans_stripped = stripped;
    stats
}

fn normalize_header_footer_key(s: &str) -> String {
    // Strip digits so page numbers embedded in running footers ("564 MSPM0 C-Series...") still match
    s.chars()
        .filter(|c| !c.is_ascii_digit())
        .collect::<String>()
        .to_ascii_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
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
            let tol = y_tolerance.max(max_h * 0.45);
            if (cy - line_cy).abs() <= tol {
                last_line.push(span.clone());
                continue;
            }
        }
        lines.push(vec![span.clone()]);
    }

    let mut result = Vec::with_capacity(lines.len());
    for mut line_spans in lines {
        line_spans.sort_by(|a, b| {
            a.bbox
                .x0
                .partial_cmp(&b.bbox.x0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        let mut bbox = line_spans[0].bbox;
        let mut max_font = line_spans[0].font_size;
        let mut bold_chars = 0usize;
        let mut total_chars = 0usize;
        let mut assembled = String::new();

        for (idx, sp) in line_spans.iter().enumerate() {
            bbox = bbox.union(&sp.bbox);
            if sp.font_size > max_font {
                max_font = sp.font_size;
            }
            let char_len = sp.text.chars().count();
            total_chars += char_len;
            if sp.is_bold {
                bold_chars += char_len;
            }

            if idx > 0 {
                let prev = &line_spans[idx - 1];
                let gap = sp.bbox.x0 - prev.bbox.x1;
                let min_font = sp.font_size.min(prev.font_size);
                let space_thresh = (min_font * 0.16).max(1.1);
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

                if (gap > space_thresh || phrase_boundary)
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
    fn test_boilerplate_detection() {
        assert!(is_boilerplate_line("www.ti.com"));
        assert!(is_boilerplate_line("Submit Document Feedback"));
        assert!(is_boilerplate_line("Return to the Summary Table."));
        assert!(is_boilerplate_line(
            "Copyright © 2026 Texas Instruments Incorporated"
        ));
        assert!(is_boilerplate_line(
            "SLASFJ6B – JULY 2025 – REVISED JANUARY 2026"
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
