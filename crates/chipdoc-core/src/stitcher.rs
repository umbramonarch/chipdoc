use crate::cleaner::{assemble_text_lines, is_boilerplate_line, slugify};
use crate::model::{ExtractedTable, FigureRegion, PageBlock, RawPageData, TextLine, TocEntry};
use crate::registers::{
    build_bitmap_summary_from_table, infer_peripheral_from_register_name,
    infer_peripheral_from_registers_heading, infer_peripheral_name,
    parse_register_heading_with_lookahead,
};
use regex::Regex;
use std::sync::OnceLock;

/// Assembles all semantic blocks (`PageBlock`) for a single page from its remaining
/// unconsumed text spans, extracted tables, and detected figures.
pub fn assemble_page_blocks(
    page: &RawPageData,
    tables: Vec<ExtractedTable>,
    figures: Vec<FigureRegion>,
    current_peripheral: &mut String,
) -> Vec<PageBlock> {
    let lines = assemble_text_lines(&page.spans, 2.5);
    let mut blocks: Vec<PageBlock> = Vec::new();

    // 1. Convert remaining text lines into Headings, RegisterHeadings, and Paragraphs
    let mut para_buf: Vec<String> = Vec::new();
    let mut para_y0: f32 = 0.0;
    let mut prev_y1: f32 = 0.0;

    let flush_para = |buf: &mut Vec<String>, y0: f32, out: &mut Vec<PageBlock>| {
        if !buf.is_empty() {
            let joined = join_paragraph_lines(buf);
            buf.clear();
            if !joined.is_empty() {
                out.push(PageBlock::Paragraph {
                    text: joined,
                    page: page.page_number,
                    y0,
                });
            }
        }
    };

    let mut line_idx = 0usize;
    while line_idx < lines.len() {
        let line = &lines[line_idx];
        let trimmed = line.text.trim();
        if trimmed.is_empty() || is_boilerplate_line(trimmed) {
            line_idx += 1;
            continue;
        }

        // Check if this line is a Register Definition Heading ("10.3.1 FSUB_0 (Offset = 400h)"
        // or "6.4.1 GPIO port mode register (GPIOx_MODER)" followed by "Address offset: 0x00")
        if let Some(reg_head) = parse_register_heading_with_lookahead(&lines, line_idx) {
            flush_para(&mut para_buf, para_y0, &mut blocks);
            let reg_periph = if current_peripheral == "SYS" {
                infer_peripheral_from_register_name(&reg_head.register_name)
                    .unwrap_or_else(|| current_peripheral.clone())
            } else {
                current_peripheral.clone()
            };
            blocks.push(PageBlock::RegisterHeading {
                section_num: reg_head.section_num,
                peripheral: reg_periph,
                register: reg_head.register_name,
                offset: reg_head.offset,
                reset: reg_head.reset,
                description: None,
                page: page.page_number,
                y0: reg_head.bbox.y0,
            });
            prev_y1 = reg_head.bbox.y1;
            line_idx += reg_head.lines_consumed.max(1);
            continue;
        }

        // Check if this line starts a prose bitfield definition ("Bits 31:0 ..." or "Bit 16 ...")
        // so each bitfield description forms its own clean paragraph block
        let is_bitfield_line = trimmed.starts_with("Bits ") || trimmed.starts_with("Bit ");
        if is_bitfield_line && !para_buf.is_empty() {
            flush_para(&mut para_buf, para_y0, &mut blocks);
        }

        // Check if this line is a numbered section heading or prominent bold heading
        if let Some((lvl, clean_heading)) = classify_heading(line) {
            flush_para(&mut para_buf, para_y0, &mut blocks);
            if lvl <= 2 {
                let inferred = infer_peripheral_name(&clean_heading);
                if inferred != "SYS" {
                    *current_peripheral = inferred;
                }
            } else if lvl == 3 {
                if let Some(inferred) = infer_peripheral_from_registers_heading(&clean_heading) {
                    *current_peripheral = inferred;
                }
            }
            blocks.push(PageBlock::Heading {
                level: lvl,
                text: clean_heading,
                page: page.page_number,
                y0: line.bbox.y0,
            });
            prev_y1 = line.bbox.y1;
            line_idx += 1;
            continue;
        }

        // Check if this line starts a new paragraph or bullet item (vertical gap > 7.5 pt or bullet prefix)
        let v_gap = if para_buf.is_empty() {
            0.0
        } else {
            line.bbox.y0 - prev_y1
        };
        if !para_buf.is_empty() && (v_gap > 8.0 || is_list_item(trimmed) || trimmed == "Note") {
            flush_para(&mut para_buf, para_y0, &mut blocks);
        }

        if para_buf.is_empty() {
            para_y0 = line.bbox.y0;
        }
        para_buf.push(trimmed.to_string());
        prev_y1 = line.bbox.y1;
        line_idx += 1;
    }
    flush_para(&mut para_buf, para_y0, &mut blocks);

    // 2. Add Tables (BitMapSummary is injected after multi-page table stitching)
    for table in tables {
        let y0 = table.bbox.y0;
        blocks.push(PageBlock::Table { table, y0 });
    }

    // 3. Add Figures
    for figure in figures {
        let y0 = figure.bbox.y0;
        blocks.push(PageBlock::Figure { figure, y0 });
    }

    // Sort all blocks on the page top-to-bottom by y0
    blocks.sort_by(|a, b| {
        a.y0()
            .partial_cmp(&b.y0())
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    blocks
}

fn classify_heading(line: &TextLine) -> Option<(usize, String)> {
    static RE_NUM_SEC: OnceLock<Regex> = OnceLock::new();
    let re = RE_NUM_SEC.get_or_init(|| {
        Regex::new(r"^(\d+(?:\.\d+){0,5})\s+([A-Z][A-Za-z0-9_/\-\s(),:&]{1,90})$")
            .expect("valid regex")
    });

    let t = line.text.trim();
    if t.starts_with("Table ") || t.starts_with("Figure ") || t.starts_with("Fig.") {
        return None;
    }

    if let Some(caps) = re.captures(t) {
        let num = caps.get(1)?.as_str();
        let rest = caps.get(2)?.as_str().trim();
        // Avoid misclassifying numeric values with units like "32 MHz" or "100 mA" or dates
        let unit_words = [
            "mhz", "khz", "hz", "ma", "ua", "na", "mv", "kv", "ns", "us", "ms", "kb", "mb", "bit",
            "bits", "pin", "pins",
        ];
        let first_word = rest
            .split_whitespace()
            .next()
            .unwrap_or("")
            .to_ascii_lowercase();
        if unit_words.contains(&first_word.as_str()) {
            return None;
        }

        if line.is_bold || line.font_size >= 10.5 {
            let dot_count = num.chars().filter(|&c| c == '.').count();
            let level = (dot_count + 1).clamp(1, 5);
            return Some((level, t.to_string()));
        }
    }

    if line.is_bold && line.font_size >= 13.5 && t.len() <= 85 {
        return Some((1, t.to_string()));
    }

    None
}

fn is_list_item(text: &str) -> bool {
    text.starts_with("• ")
        || text.starts_with("- ")
        || text.starts_with("* ")
        || (text.starts_with('(')
            && text.len() > 4
            && text.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
            && text.chars().nth(2) == Some(')'))
}

fn join_paragraph_lines(lines: &[String]) -> String {
    if lines.is_empty() {
        return String::new();
    }
    if lines.len() == 1 && lines[0] == "Note" {
        return "> **Note:**".to_string();
    }

    let mut out = String::new();
    for (i, line) in lines.iter().enumerate() {
        let clean = if let Some(rest) = line.strip_prefix("• ") {
            format!("- {rest}")
        } else {
            line.clone()
        };

        if i == 0 {
            out.push_str(&clean);
        } else if clean.starts_with("- ") {
            out.push('\n');
            out.push_str(&clean);
        } else if out.ends_with('-') && clean.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        {
            // Hyphenated word wrap across lines
            out.push_str(&clean);
        } else {
            out.push(' ');
            out.push_str(&clean);
        }
    }
    out
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InterveningPara {
    Noise,
    Footnote,
}

fn classify_intervening_paragraph(
    text: &str,
    para_page: usize,
    t1_start: usize,
    t1_end: usize,
) -> Option<InterveningPara> {
    let trimmed = text.trim();
    if trimmed.len() > 140 || para_page < t1_start || para_page > t1_end + 1 {
        return None;
    }

    static RE_BARE_PAGE: OnceLock<Regex> = OnceLock::new();
    static RE_CONT_BANNER: OnceLock<Regex> = OnceLock::new();
    static RE_FOOTNOTE: OnceLock<Regex> = OnceLock::new();

    let re_page = RE_BARE_PAGE.get_or_init(|| Regex::new(r"^\d{1,4}$").expect("valid regex"));
    let re_cont =
        RE_CONT_BANNER.get_or_init(|| Regex::new(r"(?i)\bcontinued\b").expect("valid regex"));
    let re_fn =
        RE_FOOTNOTE.get_or_init(|| Regex::new(r"^(?:\(\d+\)|\d+\.)\s+").expect("valid regex"));

    if re_page.is_match(trimmed) || re_cont.is_match(trimmed) {
        Some(InterveningPara::Noise)
    } else if re_fn.is_match(trimmed) || (para_page == t1_end && !trimmed.contains('\n')) {
        Some(InterveningPara::Footnote)
    } else {
        None
    }
}

/// Stitches multi-page `(continued)` tables across consecutive pages inside a block stream.
/// Returns the number of multi-page table continuations stitched.
pub fn stitch_multipage_tables(blocks: &mut Vec<PageBlock>) -> usize {
    let mut stitched_count = 0usize;
    let mut i = 0;
    let mut pending_footnotes: Vec<PageBlock> = Vec::new();

    while i + 1 < blocks.len() {
        let (t1_start, t1_end) = match &blocks[i] {
            PageBlock::Table { table, .. } => (table.page_start, table.page_end),
            _ => {
                i += 1;
                continue;
            }
        };

        // Look ahead across 0..=2 eligible intervening Paragraph blocks (`i + 1`, `i + 2`, `i + 3`)
        let max_look = (i + 3).min(blocks.len() - 1);
        let mut matched_table_idx: Option<usize> = None;
        let mut intervening_kinds: Vec<InterveningPara> = Vec::new();

        for cand_idx in (i + 1)..=max_look {
            match &blocks[cand_idx] {
                PageBlock::Table { table: t2, .. } => {
                    let can_merge = match &blocks[i] {
                        PageBlock::Table { table: t1, .. } => tables_are_continuation(t1, t2),
                        _ => false,
                    };
                    if can_merge {
                        matched_table_idx = Some(cand_idx);
                    }
                    break;
                }
                PageBlock::Paragraph { text, page, .. } if cand_idx < max_look => {
                    if let Some(kind) =
                        classify_intervening_paragraph(text, *page, t1_start, t1_end)
                    {
                        intervening_kinds.push(kind);
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }

        if let Some(next_table_idx) = matched_table_idx {
            // Drain any intervening paragraphs between `i` and `next_table_idx`:
            // drop pure noise and keep footnotes to place right after the stitched table.
            for (blk, kind) in blocks.drain((i + 1)..next_table_idx).zip(intervening_kinds) {
                if kind == InterveningPara::Footnote {
                    pending_footnotes.push(blk);
                }
            }
            // Now `t2` is at `i + 1`
            let PageBlock::Table { table: t2, .. } = blocks.remove(i + 1) else {
                unreachable!()
            };
            if let PageBlock::Table { table: t1, .. } = &mut blocks[i] {
                let target_cols = t1.headers.len();
                for mut row in t2.rows {
                    if target_cols > 0 {
                        if row.len() < target_cols {
                            row.resize(target_cols, String::new());
                        } else if row.len() > target_cols {
                            row.truncate(target_cols);
                        }
                    }
                    t1.rows.push(row);
                }
                t1.page_end = t1.page_end.max(t2.page_end);
                stitched_count += 1;
            }
            // Stay on `i` in case the table continues onto a subsequent page
            continue;
        }

        if !pending_footnotes.is_empty() {
            let fn_count = pending_footnotes.len();
            blocks.splice((i + 1)..(i + 1), pending_footnotes.drain(..));
            i += 1 + fn_count;
        } else {
            i += 1;
        }
    }

    if !pending_footnotes.is_empty() && i < blocks.len() {
        blocks.splice((i + 1)..(i + 1), pending_footnotes);
    }

    // After stitching multi-page tables, inject a 1-line BitMapSummary right before each Field Descriptions table
    let mut k = 0;
    while k < blocks.len() {
        let maybe_summary = match &blocks[k] {
            PageBlock::Table { table, y0 } => build_bitmap_summary_from_table(table)
                .map(|summary| (summary, table.page_start, (*y0 - 0.5).max(0.0))),
            _ => None,
        };
        if let Some((summary, page, y0)) = maybe_summary {
            blocks.insert(k, PageBlock::BitMapSummary { summary, page, y0 });
            k += 2;
        } else {
            k += 1;
        }
    }

    stitched_count
}

fn tables_are_continuation(t1: &ExtractedTable, t2: &ExtractedTable) -> bool {
    if t2.page_start > t1.page_end + 1 {
        return false;
    }
    let cols_compatible_loose = t1.headers.len() >= 2
        && t2.headers.len() >= 2
        && t1.headers.len().abs_diff(t2.headers.len()) <= 2;

    // Check explicit "(continued)" caption on t2
    if let Some(cap2) = &t2.caption {
        let low2 = cap2.to_ascii_lowercase();
        if (low2.contains("(continued)") || low2.contains("continued"))
            && (t1.headers.len() == t2.headers.len() || cols_compatible_loose)
        {
            return true;
        }
        // Same "Table X-Y" prefix
        if let Some(cap1) = &t1.caption {
            let p1 = extract_table_prefix(cap1);
            let p2 = extract_table_prefix(cap2);
            if !p1.is_empty()
                && p1 == p2
                && (t1.headers.len() == t2.headers.len() || cols_compatible_loose)
            {
                return true;
            }
        }
    } else if t1.headers == t2.headers && !t1.headers.is_empty() && t2.page_start == t1.page_end + 1
    {
        return true;
    }

    false
}

fn extract_table_prefix(caption: &str) -> String {
    static RE_PREFIX: OnceLock<Regex> = OnceLock::new();
    let re =
        RE_PREFIX.get_or_init(|| Regex::new(r"^(Table\s+\d+(?:[-\.]\d+)*)").expect("valid regex"));
    re.captures(caption.trim())
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_ascii_lowercase())
        .unwrap_or_default()
}

fn extract_leading_chapter_num(title: &str) -> Option<&str> {
    let first_tok = title.split_whitespace().next()?;
    let chap = first_tok.split('.').next()?;
    if !chap.is_empty() && chap.chars().all(|c| c.is_ascii_digit()) {
        Some(chap)
    } else {
        None
    }
}

/// Computes top-level section shards (`TocEntry` at `level == 0`) with inclusive `[start_page, end_page]`
/// ranges covering all converted pages. Falls back to synthetic 25-page shards if the PDF has no bookmarks.
pub fn compute_section_shards(toc: &[TocEntry], total_pages: usize) -> Vec<TocEntry> {
    let top_entries: Vec<TocEntry> = toc
        .iter()
        .filter(|e| e.level == 0 && e.start_page >= 1 && e.start_page <= total_pages)
        .cloned()
        .collect();

    if top_entries.is_empty() {
        let chunk_size = 25usize;
        let mut shards = Vec::new();
        let mut start = 1usize;
        let mut idx = 1usize;
        while start <= total_pages {
            let end = (start + chunk_size - 1).min(total_pages);
            let title = if total_pages <= chunk_size {
                "Document Content".to_string()
            } else {
                format!("Pages {start} to {end}")
            };
            shards.push(TocEntry {
                id: format!("{idx:02}"),
                level: 0,
                slug: format!("{idx:02}-{}", slugify(&title)),
                title,
                start_page: start,
                end_page: end,
            });
            start = end + 1;
            idx += 1;
        }
        return shards;
    }

    // Merge consecutive top-level entries that start on the same page so shard page ranges are strictly disjoint.
    let mut merged: Vec<TocEntry> = Vec::new();
    for entry in top_entries {
        if let Some(last) = merged.last_mut() {
            if entry.start_page <= last.start_page {
                last.title = format!("{} / {}", last.title, entry.title);
                continue;
            }
        }
        merged.push(entry);
    }

    for i in 0..merged.len() {
        let next_start = merged
            .get(i + 1)
            .map(|e| e.start_page)
            .unwrap_or(total_pages + 1);
        merged[i].end_page = if next_start > merged[i].start_page {
            let shard_chap = extract_leading_chapter_num(&merged[i].title);
            let has_child_on_next_start = next_start <= total_pages
                && toc.iter().any(|e| {
                    e.level > 0
                        && e.start_page == next_start
                        && shard_chap.is_some()
                        && extract_leading_chapter_num(&e.title) == shard_chap
                });
            if has_child_on_next_start {
                next_start.min(total_pages)
            } else {
                (next_start - 1).min(total_pages)
            }
        } else {
            merged[i].start_page.min(total_pages)
        };
        merged[i].id = format!("{:02}", i + 1);
        merged[i].slug = format!("{:02}-{}", i + 1, slugify(&merged[i].title));
    }

    // If page 1 is before the first bookmark, prepend a Frontmatter shard
    if let Some(first) = merged.first() {
        if first.start_page > 1 {
            merged.insert(
                0,
                TocEntry {
                    id: "00".to_string(),
                    level: 0,
                    title: "Frontmatter and Overview".to_string(),
                    start_page: 1,
                    end_page: first.start_page - 1,
                    slug: "00-frontmatter-and-overview".to_string(),
                },
            );
        }
    }

    merged
}
