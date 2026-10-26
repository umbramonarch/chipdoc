use crate::cleaner::{assemble_text_lines, is_boilerplate_line, slugify};
use crate::model::{ExtractedTable, FigureRegion, PageBlock, RawPageData, TextLine, TocEntry};
use crate::registers::{
    build_bitmap_summary_from_table, infer_peripheral_name, parse_register_heading_with_lookahead,
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

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.text.trim();
        if trimmed.is_empty() || is_boilerplate_line(trimmed) {
            continue;
        }

        // Check if this line is a Register Definition Heading ("10.3.1 FSUB_0 (Offset = 400h)"
        // or "6.4.1 GPIO port mode register (GPIOx_MODER)" followed by "Address offset: 0x00")
        if let Some(reg_head) = parse_register_heading_with_lookahead(&lines, line_idx) {
            flush_para(&mut para_buf, para_y0, &mut blocks);
            blocks.push(PageBlock::RegisterHeading {
                section_num: reg_head.section_num,
                peripheral: current_peripheral.clone(),
                register: reg_head.register_name,
                offset: reg_head.offset,
                reset: reg_head.reset,
                description: None,
                page: page.page_number,
                y0: line.bbox.y0,
            });
            prev_y1 = line.bbox.y1;
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
            if lvl <= 3 {
                let inferred = infer_peripheral_name(&clean_heading);
                if inferred != "SYS" {
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

/// Stitches multi-page `(continued)` tables across consecutive pages inside a block stream.
/// Returns the number of multi-page table continuations stitched.
pub fn stitch_multipage_tables(blocks: &mut Vec<PageBlock>) -> usize {
    let mut stitched_count = 0usize;
    let mut i = 0;

    while i + 1 < blocks.len() {
        let is_table_i = matches!(blocks[i], PageBlock::Table { .. });
        if !is_table_i {
            i += 1;
            continue;
        }

        // Look ahead to `i + 1` (or `i + 2` if separated only by a small continuation note)
        let next_idx = i + 1;
        let can_merge = match (&blocks[i], &blocks[next_idx]) {
            (PageBlock::Table { table: t1, .. }, PageBlock::Table { table: t2, .. }) => {
                tables_are_continuation(t1, t2)
            }
            _ => false,
        };

        if can_merge {
            let PageBlock::Table { table: t2, .. } = blocks.remove(next_idx) else {
                unreachable!()
            };
            if let PageBlock::Table { table: t1, .. } = &mut blocks[i] {
                t1.rows.extend(t2.rows);
                t1.page_end = t1.page_end.max(t2.page_end);
                stitched_count += 1;
            }
            // Stay on `i` in case the table continues onto a third page
            continue;
        }

        i += 1;
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
    // Check explicit "(continued)" caption on t2
    if let Some(cap2) = &t2.caption {
        let low2 = cap2.to_ascii_lowercase();
        if (low2.contains("(continued)") || low2.contains("continued"))
            && t1.headers.len() == t2.headers.len()
        {
            return true;
        }
        // Same "Table X-Y" prefix
        if let Some(cap1) = &t1.caption {
            if extract_table_prefix(cap1) == extract_table_prefix(cap2)
                && !extract_table_prefix(cap1).is_empty()
                && t1.headers.len() == t2.headers.len()
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
            (next_start - 1).min(total_pages)
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
