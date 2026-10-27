use crate::cleaner::{assemble_text_lines, slugify};
use crate::model::{BBox, ExtractedTable, FigureRegion, RawPageData, TextLine};
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

    // Collect graphic/image bounding boxes that are not part of already-extracted tables
    // and are not full-page border frames.
    let page_area = page.width * page.height;
    let mut candidate_art_boxes: Vec<BBox> = Vec::new();
    for &b in page.graphic_bboxes.iter().chain(page.image_bboxes.iter()) {
        if b.area() > page_area * 0.88 || (b.width() < 2.0 && b.height() < 2.0) {
            continue;
        }
        // Exclude top/bottom running header/footer rules
        if b.y1 <= page.height * 0.085 || b.y0 >= page.height * 0.895 {
            continue;
        }
        // Exclude full-width thin divider rules
        if b.width() > page.width * 0.68 && b.height() <= 3.5 {
            continue;
        }
        // Exclude paths inside extracted tables
        if tables.iter().any(|t| t.bbox.overlaps(&b, 2.0)) {
            continue;
        }
        candidate_art_boxes.push(b);
    }

    // Include short non-table grid segments so diagrams drawn with orthogonal stroked segments are detected
    for s in &page.grid_segments {
        let seg_w = (s.x1 - s.x0).abs();
        let seg_h = (s.y1 - s.y0).abs();
        if seg_w.max(seg_h) < 4.0 {
            continue;
        }
        if seg_w <= page.width * 0.65 && s.y0 >= page.height * 0.085 && s.y1 <= page.height * 0.895
        {
            let sb = BBox::new(s.x0, s.y0, s.x1, s.y1);
            if !tables.iter().any(|t| t.bbox.overlaps(&sb, 2.0)) {
                candidate_art_boxes.push(sb);
            }
        }
    }

    let mut figures = Vec::new();
    let mut claimed_regions: Vec<BBox> = Vec::new();

    for (line_idx, line) in lines.iter().enumerate() {
        if tables
            .iter()
            .any(|tb| tb.bbox.contains_center_of(&line.bbox, 3.0))
        {
            continue;
        }

        let Some((fig_num, caption_text)) = parse_valid_figure_caption(line) else {
            continue;
        };

        // Determine the vertical search window above the caption (`[y_limit_top, line.bbox.y0]`)
        // Stop at any preceding heading, table, figure caption, body paragraph, or already-claimed figure region.
        let mut y_limit_top = page.height * 0.085;
        for prev in lines[..line_idx].iter().rev() {
            let prev_t = prev.text.trim();
            if parse_valid_figure_caption(prev).is_some()
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
        for cr in &claimed_regions {
            if cr.y1 <= line.bbox.y0 + 6.0 {
                y_limit_top = y_limit_top.max(cr.y1 + 4.0);
            }
        }

        // Gather unclaimed art boxes inside `[y_limit_top, line.bbox.y0 + 8.0]` (caption BELOW figure)
        // and cluster by vertical proximity to the caption (`max_gap = 42.0 pt`).
        let mut above_boxes: Vec<BBox> = candidate_art_boxes
            .iter()
            .copied()
            .filter(|b| {
                b.y0 >= y_limit_top - 6.0
                    && b.y1 <= line.bbox.y0 + 8.0
                    && !claimed_regions.iter().any(|cr| cr.overlaps(b, 1.0))
            })
            .collect();
        above_boxes.sort_by(|a, b| b.y1.partial_cmp(&a.y1).unwrap_or(std::cmp::Ordering::Equal));

        let mut art_region: Option<BBox> = None;
        if let Some(&first) = above_boxes.first() {
            if line.bbox.y0 - first.y1 <= 45.0 {
                let mut acc = first;
                for &b in above_boxes.iter().skip(1) {
                    if acc.y0 - b.y1 <= 42.0 {
                        acc = acc.union(&b);
                    }
                }
                art_region = Some(acc);
            }
        }

        // Include short diagram labels within `[region.y0 - 12.0, region.y1 + 12.0]` if art exists above
        if let Some(mut region) = art_region {
            for prev in &lines[..line_idx] {
                if prev.bbox.y0 >= y_limit_top
                    && prev.bbox.y1 <= line.bbox.y0
                    && prev.bbox.y0 >= region.y0 - 12.0
                    && prev.bbox.y1 <= region.y1 + 12.0
                    && prev.text.len() <= 45
                {
                    region = region.union(&prev.bbox);
                }
            }
            if region.height() < 20.0 || region.width() < 35.0 {
                art_region = None;
            } else {
                art_region = Some(region);
            }
        }

        // Fallback: if no art region exists above the caption, search BELOW the caption
        // (handles manuals where `Figure X.` captions are placed at the top of the diagram).
        if art_region.is_none() {
            let mut y_limit_bottom = page.height * 0.895;
            for next in lines.iter().skip(line_idx + 1) {
                let next_t = next.text.trim();
                if parse_valid_figure_caption(next).is_some()
                    || next_t.starts_with("Table ")
                    || is_prose_or_heading(next_t, next.is_bold, next.font_size)
                {
                    y_limit_bottom = y_limit_bottom.min(next.bbox.y0 - 2.0);
                    break;
                }
            }
            for tb in tables {
                if tb.bbox.y0 >= line.bbox.y1 - 4.0 {
                    y_limit_bottom = y_limit_bottom.min(tb.bbox.y0 - 4.0);
                }
            }
            for cr in &claimed_regions {
                if cr.y0 >= line.bbox.y1 - 6.0 {
                    y_limit_bottom = y_limit_bottom.min(cr.y0 - 4.0);
                }
            }

            let mut below_boxes: Vec<BBox> = candidate_art_boxes
                .iter()
                .copied()
                .filter(|b| {
                    b.y0 >= line.bbox.y0 - 6.0
                        && b.y1 <= y_limit_bottom + 6.0
                        && !claimed_regions.iter().any(|cr| cr.overlaps(b, 1.0))
                })
                .collect();
            below_boxes
                .sort_by(|a, b| a.y0.partial_cmp(&b.y0).unwrap_or(std::cmp::Ordering::Equal));

            let mut below_region: Option<BBox> = None;
            if let Some(&first) = below_boxes.first() {
                if first.y0 - line.bbox.y1 <= 45.0 {
                    let mut acc = first;
                    for &b in below_boxes.iter().skip(1) {
                        if b.y0 - acc.y1 <= 42.0 {
                            acc = acc.union(&b);
                        }
                    }
                    below_region = Some(acc);
                }
            }

            if let Some(mut region) = below_region {
                for next in lines.iter().skip(line_idx + 1) {
                    if next.bbox.y0 >= line.bbox.y1
                        && next.bbox.y1 <= y_limit_bottom
                        && next.bbox.y0 >= region.y0 - 12.0
                        && next.bbox.y1 <= region.y1 + 12.0
                        && next.text.len() <= 55
                    {
                        region = region.union(&next.bbox);
                    }
                }
                if region.height() >= 20.0 && region.width() >= 35.0 {
                    art_region = Some(region);
                }
            }
        }

        if let Some(region) = art_region {
            if region.height() >= 20.0 && region.width() >= 35.0 {
                let full_bbox = region
                    .union(&line.bbox)
                    .expand(8.0, 6.0, page.width, page.height);
                claimed_regions.push(full_bbox);

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

fn parse_valid_figure_caption(line: &TextLine) -> Option<(&str, String)> {
    let t = line.text.trim();
    if t.contains("...") || t.contains(". . .") {
        return None;
    }

    static RE_FIG_CAP: OnceLock<Regex> = OnceLock::new();
    static RE_FIG_VERB: OnceLock<Regex> = OnceLock::new();
    let re = RE_FIG_CAP.get_or_init(|| {
        Regex::new(r"^(?:Figure|Fig\.)\s+(\d+(?:[-\.]\d+)*)(?:([.:])\s*(.*)|\s+(.+))$")
            .expect("valid regex")
    });
    let re_verb = RE_FIG_VERB.get_or_init(|| {
        Regex::new(
            r"^(?i)(?:shows?|is|are|was|were|illustrates?|depicts?|describes?|gives?|lists?|provides?|represents?|can|may|in|on|at|to|for|with|and|or)\b",
        )
        .expect("valid regex")
    });

    let caps = re.captures(t)?;
    if caps.get(2).is_none() && !line.is_bold {
        return None;
    }

    let fig_num = caps.get(1)?.as_str();
    let rest = caps
        .get(3)
        .or_else(|| caps.get(4))
        .map(|m| m.as_str().trim())
        .unwrap_or("");

    if rest.is_empty()
        || rest.starts_with(')')
        || rest.starts_with(',')
        || rest.starts_with(';')
        || rest.chars().next().is_some_and(|c| c.is_ascii_lowercase())
        || re_verb.is_match(rest)
    {
        return None;
    }

    Some((fig_num, t.to_string()))
}

fn is_prose_or_heading(text: &str, is_bold: bool, font_size: f32) -> bool {
    static RE_SEC_HEAD: OnceLock<Regex> = OnceLock::new();
    let re =
        RE_SEC_HEAD.get_or_init(|| Regex::new(r"^\d+(?:\.\d+)*\s+[A-Z]").expect("valid regex"));

    if (is_bold || font_size >= 10.0) && re.is_match(text) {
        return true;
    }
    if is_bold && font_size >= 9.5 && text.len() >= 12 {
        return true;
    }
    if text.starts_with("• ")
        || text.starts_with("- ")
        || text.starts_with("Note")
        || text.starts_with("CAUTION")
        || text.starts_with("WARNING")
    {
        return true;
    }
    text.len() >= 62 && text.split_whitespace().count() >= 9
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::TextSpan;

    #[test]
    fn test_reject_prose_figure_reference() {
        let mut page = RawPageData {
            page_number: 1,
            width: 612.0,
            height: 792.0,
            spans: vec![
                TextSpan {
                    text:
                        "Figure 4-1 shows the internal clock distribution network for the device."
                            .to_string(),
                    bbox: BBox::new(72.0, 220.0, 500.0, 232.0),
                    font_size: 10.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
                TextSpan {
                    text: "Figure 89.".to_string(),
                    bbox: BBox::new(72.0, 240.0, 140.0, 252.0),
                    font_size: 10.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
            ],
            grid_segments: Vec::new(),
            graphic_bboxes: vec![BBox::new(100.0, 120.0, 460.0, 205.0)],
            image_bboxes: Vec::new(),
        };

        let (figures, stats) = detect_page_figures(&mut page, &[]);
        assert!(figures.is_empty());
        assert_eq!(stats.figures_detected, 0);
        assert!(page.spans.iter().all(|s| !s.consumed));
    }

    #[test]
    fn test_stacked_caption_above_figures_do_not_steal_art() {
        let mut page = RawPageData {
            page_number: 2,
            width: 612.0,
            height: 792.0,
            spans: vec![
                TextSpan {
                    text: "Figure 1-1. Top Level Block Diagram".to_string(),
                    bbox: BBox::new(100.0, 100.0, 320.0, 112.0),
                    font_size: 9.5,
                    is_bold: true,
                    is_mono: false,
                    consumed: false,
                },
                TextSpan {
                    text: "CPU_CORE".to_string(),
                    bbox: BBox::new(180.0, 160.0, 240.0, 170.0),
                    font_size: 8.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
                TextSpan {
                    text: "Figure 1-2. Clock Tree Diagram".to_string(),
                    bbox: BBox::new(100.0, 260.0, 300.0, 272.0),
                    font_size: 9.5,
                    is_bold: true,
                    is_mono: false,
                    consumed: false,
                },
                TextSpan {
                    text: "PLL_CLK".to_string(),
                    bbox: BBox::new(180.0, 320.0, 235.0, 330.0),
                    font_size: 8.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
            ],
            grid_segments: Vec::new(),
            graphic_bboxes: vec![
                BBox::new(100.0, 120.0, 450.0, 240.0),
                BBox::new(100.0, 280.0, 450.0, 400.0),
            ],
            image_bboxes: Vec::new(),
        };

        let (figures, stats) = detect_page_figures(&mut page, &[]);
        assert_eq!(stats.figures_detected, 2);
        assert_eq!(figures.len(), 2);
        assert_eq!(figures[0].id, "fig-1-1");
        assert!(figures[0].bbox.y1 < 255.0);
        assert_eq!(figures[1].id, "fig-1-2");
        assert!(figures[1].bbox.y0 > 250.0);
        assert!(figures[1].bbox.y1 >= 400.0);
    }

    #[test]
    fn test_body_prose_below_figure_not_swallowed() {
        let mut page = RawPageData {
            page_number: 3,
            width: 612.0,
            height: 792.0,
            spans: vec![
                TextSpan {
                    text: "Figure 2-1. Timing Waveform".to_string(),
                    bbox: BBox::new(100.0, 100.0, 280.0, 112.0),
                    font_size: 9.5,
                    is_bold: true,
                    is_mono: false,
                    consumed: false,
                },
                TextSpan {
                    text: "tSU".to_string(),
                    bbox: BBox::new(150.0, 155.0, 175.0, 165.0),
                    font_size: 8.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
                // Wrapped prose line without trailing period (>= 62 chars and >= 9 words)
                TextSpan {
                    text: "The timing parameters shown in the diagram above are guaranteed by design across"
                        .to_string(),
                    bbox: BBox::new(72.0, 250.0, 520.0, 262.0),
                    font_size: 10.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
                // Short continuation line after the wrapped prose line
                TextSpan {
                    text: "all operating temperatures.".to_string(),
                    bbox: BBox::new(72.0, 265.0, 210.0, 277.0),
                    font_size: 10.0,
                    is_bold: false,
                    is_mono: false,
                    consumed: false,
                },
            ],
            grid_segments: Vec::new(),
            graphic_bboxes: vec![BBox::new(100.0, 120.0, 450.0, 220.0)],
            image_bboxes: Vec::new(),
        };

        let (figures, _stats) = detect_page_figures(&mut page, &[]);
        assert_eq!(figures.len(), 1);
        assert!(figures[0].bbox.y1 < 240.0);
        assert!(!page.spans[2].consumed);
        assert!(!page.spans[3].consumed);
    }
}
