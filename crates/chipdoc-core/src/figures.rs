use crate::cleaner::{assemble_text_lines, slugify};
use crate::model::{BBox, ExtractedTable, FigureRegion, RawPageData};
use regex::Regex;
use std::sync::OnceLock;

/// Statistics returned by figure detection on a page.
#[derive(Debug, Default, Clone, Copy)]
pub struct FigureDetectStats {
    pub figures_detected: usize,
    pub spans_suppressed: usize,
}

/// Detects hardware figures (pinouts, block diagrams, timing diagrams, package drawings)
/// on a page by pairing `Figure X-Y.` captions with clustered vector/raster bounding boxes
/// above (or below) the caption, and suppresses floating text spans inside the figure region
/// so they are not dumped as scrambled text into the Markdown body.
pub fn detect_page_figures(
    page: &mut RawPageData,
    tables: &[ExtractedTable],
) -> (Vec<FigureRegion>, FigureDetectStats) {
    let mut stats = FigureDetectStats::default();
    let lines = assemble_text_lines(&page.spans, 2.5);
    if lines.is_empty() {
        return (Vec::new(), stats);
    }

    static RE_FIG_CAP: OnceLock<Regex> = OnceLock::new();
    let re_fig = RE_FIG_CAP.get_or_init(|| {
        Regex::new(r"^(?:Figure|Fig\.)\s+(\d+(?:[-\.]\d+)*)\b[.:]?\s*(.*)$").expect("valid regex")
    });

    // Collect graphic/image bounding boxes that are not part of already-extracted tables
    // and are not full-page border frames.
    let page_area = page.width * page.height;
    let mut candidate_art_boxes: Vec<BBox> = Vec::new();
    for &b in page.graphic_bboxes.iter().chain(page.image_bboxes.iter()) {
        if b.area() > page_area * 0.88 || (b.width() < 2.0 && b.height() < 2.0) {
            continue;
        }
        // Exclude top/bottom running header/footer rules
        if b.y1 <= page.height * 0.075 || b.y0 >= page.height * 0.915 {
            continue;
        }
        // Exclude paths inside extracted tables
        if tables.iter().any(|t| t.bbox.overlaps(&b, 2.0)) {
            continue;
        }
        candidate_art_boxes.push(b);
    }

    let mut figures = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        let Some(caps) = re_fig.captures(line.text.trim()) else {
            continue;
        };

        let fig_num = caps.get(1).map(|m| m.as_str()).unwrap_or("1");
        let caption_text = line.text.trim().to_string();

        // Determine the vertical search window above the caption (`[y_limit_top, line.bbox.y0]`)
        // Stop at any preceding heading, table, figure caption, or body paragraph (> 65 chars).
        let mut y_limit_top = page.height * 0.075;
        for prev in lines[..line_idx].iter().rev() {
            let prev_t = prev.text.trim();
            if re_fig.is_match(prev_t)
                || prev_t.starts_with("Table ")
                || is_prose_or_heading(prev_t, prev.is_bold, prev.font_size)
            {
                y_limit_top = y_limit_top.max(prev.bbox.y1 + 2.0);
                break;
            }
        }
        for tb in tables {
            if tb.bbox.y1 <= line.bbox.y0 + 4.0 {
                y_limit_top = y_limit_top.max(tb.bbox.y1 + 4.0);
            }
        }

        // Gather all art boxes inside `[y_limit_top, line.bbox.y0 + 4.0]`
        let mut art_region: Option<BBox> = None;
        for b in &candidate_art_boxes {
            if b.y0 >= y_limit_top - 6.0 && b.y1 <= line.bbox.y0 + 8.0 {
                art_region = Some(match art_region {
                    Some(acc) => acc.union(b),
                    None => *b,
                });
            }
        }

        // Also include short diagram labels inside `[y_limit_top, line.bbox.y0]` if art exists
        if let Some(mut region) = art_region {
            for prev in &lines[..line_idx] {
                if prev.bbox.y0 >= y_limit_top
                    && prev.bbox.y1 <= line.bbox.y0
                    && prev.text.len() <= 45
                {
                    region = region.union(&prev.bbox);
                }
            }

            if region.height() >= 20.0 && region.width() >= 35.0 {
                let full_bbox = region
                    .union(&line.bbox)
                    .expand(8.0, 6.0, page.width, page.height);

                // Suppress all text spans inside the figure region (including the caption, since we emit the caption cleanly)
                for sp in page.spans.iter_mut() {
                    if !sp.consumed && full_bbox.contains_center_of(&sp.bbox, 2.0) {
                        sp.consumed = true;
                        stats.spans_suppressed += 1;
                    }
                }

                let fig_slug = slugify(&caption_text);
                let fig_id = format!("fig-{}", fig_num.replace('.', "-"));
                let rel_path = format!(
                    "assets/figures/p{:03}-{}.png",
                    page.page_number,
                    if fig_slug.len() > 48 {
                        &fig_slug[..48]
                    } else {
                        &fig_slug
                    }
                );

                stats.figures_detected += 1;
                figures.push(FigureRegion {
                    id: fig_id,
                    caption: caption_text,
                    page_number: page.page_number,
                    bbox: full_bbox,
                    image_rel_path: Some(rel_path),
                    section_file: String::new(),
                    line_number: 0,
                });
            }
        }
    }

    (figures, stats)
}

fn is_prose_or_heading(text: &str, is_bold: bool, font_size: f32) -> bool {
    static RE_SEC_HEAD: OnceLock<Regex> = OnceLock::new();
    let re =
        RE_SEC_HEAD.get_or_init(|| Regex::new(r"^\d+(?:\.\d+)*\s+[A-Z]").expect("valid regex"));

    if (is_bold || font_size >= 11.0) && re.is_match(text) {
        return true;
    }
    // Full-width prose sentences end with period/colon and are > 68 chars
    text.len() >= 68 && (text.ends_with('.') || text.ends_with(':'))
}
