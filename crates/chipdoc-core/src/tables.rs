use crate::cleaner::{assemble_text_lines, normalize_technical_text};
use crate::model::{BBox, ExtractedTable, GridSegment, RawPageData, TextLine, TextSpan};
use regex::Regex;
use std::sync::OnceLock;

/// Extracts structured tables from a single page using vector grid geometry
/// (`GridSegment` horizontal/vertical rules) and text span coordinate alignment.
/// Marks all `TextSpan`s consumed by extracted tables (and their captions) as `consumed = true`.
pub fn extract_page_tables(page: &mut RawPageData) -> Vec<ExtractedTable> {
    let lines = assemble_text_lines(&page.spans, 2.5);
    if lines.is_empty() {
        return Vec::new();
    }

    // 1. Cluster horizontal grid segments into candidate table regions
    let mut candidate_regions =
        detect_grid_table_regions(&page.grid_segments, &lines, page.width, page.height);

    // Sort candidate regions top-to-bottom
    candidate_regions.sort_by(|a, b| {
        a.bbox
            .y0
            .partial_cmp(&b.bbox.y0)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let mut extracted = Vec::new();
    for (idx, cand) in candidate_regions.into_iter().enumerate() {
        if let Some(table) = build_table_from_region(page, &lines, cand, idx + 1) {
            extracted.push(table);
        }
    }

    extracted
}

#[derive(Debug, Clone)]
struct CandidateTableRegion {
    bbox: BBox,
    h_rules: Vec<f32>,
    v_rules: Vec<f32>,
    caption: Option<String>,
    caption_bbox: Option<BBox>,
}

fn is_table_caption(text: &str) -> bool {
    static RE_TAB_CAP: OnceLock<Regex> = OnceLock::new();
    let re =
        RE_TAB_CAP.get_or_init(|| Regex::new(r"^Table\s+\d+(?:[-\.]\d+)*\b").expect("valid regex"));
    re.is_match(text.trim())
}

/// Groups horizontal and vertical vector segments into distinct table regions on the page.
fn detect_grid_table_regions(
    segments: &[GridSegment],
    lines: &[TextLine],
    page_w: f32,
    page_h: f32,
) -> Vec<CandidateTableRegion> {
    let min_h_len = (page_w * 0.18).max(65.0);
    let top_margin = page_h * 0.065;
    let bottom_margin = page_h * 0.925;

    // Collect raw horizontal segments outside running header/footer margins, including per-cell border segments
    let mut raw_h: Vec<GridSegment> = segments
        .iter()
        .copied()
        .filter(|s| {
            s.is_horizontal && (s.x1 - s.x0) >= 6.0 && s.y0 >= top_margin && s.y1 <= bottom_margin
        })
        .collect();

    if raw_h.len() < 2 {
        return Vec::new();
    }

    // Merge collinear horizontal cell-border segments on the same Y coordinate BEFORE filtering by min_h_len.
    // This handles XSL-FO / FrameMaker tables where a 15-column table's horizontal rule is drawn as 15 adjacent cell segments.
    raw_h.sort_by(|a, b| {
        a.y0.partial_cmp(&b.y0)
            .unwrap_or(std::cmp::Ordering::Equal)
            .then_with(|| a.x0.partial_cmp(&b.x0).unwrap_or(std::cmp::Ordering::Equal))
    });

    let mut merged_h: Vec<GridSegment> = Vec::new();
    for seg in raw_h {
        if let Some(last) = merged_h.last_mut() {
            if (seg.y0 - last.y0).abs() <= 1.8 && seg.x0 <= last.x1 + 6.0 {
                last.x1 = last.x1.max(seg.x1);
                continue;
            }
        }
        merged_h.push(seg);
    }

    let mut h_segs: Vec<GridSegment> = merged_h
        .into_iter()
        .filter(|s| (s.x1 - s.x0) >= min_h_len)
        .collect();

    if h_segs.len() < 2 {
        return Vec::new();
    }

    h_segs.sort_by(|a, b| a.y0.partial_cmp(&b.y0).unwrap_or(std::cmp::Ordering::Equal));

    // Group horizontal segments into vertically connected table clusters
    let mut clusters: Vec<Vec<GridSegment>> = Vec::new();
    for seg in h_segs {
        if let Some(last_cluster) = clusters.last_mut() {
            let prev_y = last_cluster.last().map(|s| s.y0).unwrap_or(seg.y0);
            let cluster_x0 = last_cluster
                .iter()
                .map(|s| s.x0)
                .fold(f32::INFINITY, f32::min);
            let cluster_x1 = last_cluster
                .iter()
                .map(|s| s.x1)
                .fold(f32::NEG_INFINITY, f32::max);
            let h_overlap = (seg.x1.min(cluster_x1) - seg.x0.max(cluster_x0)).max(0.0);

            // Check if there is a "Table X-Y" caption, "Figure X-Y" caption, or section heading between prev_y and seg.y0
            // which indicates a new table or section started below the previous one.
            let has_intervening_caption = lines.iter().any(|l| {
                l.bbox.center_y() > prev_y + 2.0
                    && l.bbox.center_y() < seg.y0 - 2.0
                    && (is_table_caption(&l.text)
                        || l.text.trim().starts_with("Figure ")
                        || is_major_heading_line(l))
            });

            if (seg.y0 - prev_y) <= 560.0 && h_overlap >= 40.0 && !has_intervening_caption {
                last_cluster.push(seg);
                continue;
            }
        }
        clusters.push(vec![seg]);
    }

    let mut regions = Vec::new();
    for cluster in clusters {
        let h_rules = dedup_coords(cluster.iter().map(|s| s.y0).collect(), 3.0);
        if h_rules.len() < 2 {
            continue;
        }

        let x0 = cluster.iter().map(|s| s.x0).fold(f32::INFINITY, f32::min);
        let x1 = cluster
            .iter()
            .map(|s| s.x1)
            .fold(f32::NEG_INFINITY, f32::max);
        let y0 = *h_rules.first().unwrap();
        let y1 = *h_rules.last().unwrap();

        if (x1 - x0) < min_h_len || (y1 - y0) < 12.0 {
            continue;
        }

        let bbox = BBox::new(x0, y0, x1, y1);

        // Collect vertical segments intersecting this table's bounding box
        let mut v_coords: Vec<f32> = segments
            .iter()
            .filter(|s| {
                !s.is_horizontal
                    && (s.y1 - s.y0) >= 6.0
                    && s.x0 >= bbox.x0 - 6.0
                    && s.x1 <= bbox.x1 + 6.0
                    && s.y1 >= bbox.y0 - 6.0
                    && s.y0 <= bbox.y1 + 6.0
            })
            .map(|s| s.x0)
            .collect();

        // Also include recurring cell-border X junctions from horizontal segments inside this table's bbox
        let mut h_junctions: Vec<f32> = Vec::new();
        for s in segments {
            if s.is_horizontal
                && (s.x1 - s.x0) >= 12.0
                && s.y0 >= bbox.y0 - 4.0
                && s.y1 <= bbox.y1 + 4.0
                && s.x0 >= bbox.x0 - 4.0
                && s.x1 <= bbox.x1 + 4.0
            {
                h_junctions.push(s.x0);
                h_junctions.push(s.x1);
            }
        }
        h_junctions.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        // Keep junction X coordinates that appear at least twice (e.g., top & bottom of a column header or rows)
        let mut j_idx = 0;
        while j_idx < h_junctions.len() {
            let base = h_junctions[j_idx];
            let mut count = 1usize;
            let mut sum = base;
            while j_idx + count < h_junctions.len()
                && (h_junctions[j_idx + count] - base).abs() <= 2.5
            {
                sum += h_junctions[j_idx + count];
                count += 1;
            }
            if count >= 2 {
                v_coords.push(sum / (count as f32));
            }
            j_idx += count;
        }

        let v_rules = dedup_coords(v_coords, 4.0);

        // Find any "Table X-Y." caption immediately above `bbox.y0` (within 45 pt)
        let mut caption = None;
        let mut caption_bbox = None;
        let mut preceded_by_figure_caption = false;
        for line in lines.iter().rev() {
            if line.bbox.y1 <= bbox.y0 + 8.0 && (bbox.y0 - line.bbox.y0) <= 45.0 {
                let lt = line.text.trim();
                if is_table_caption(lt) {
                    caption = Some(line.text.clone());
                    caption_bbox = Some(line.bbox);
                    break;
                }
                if lt.starts_with("Figure ") || lt.starts_with("Fig.") {
                    preceded_by_figure_caption = true;
                    break;
                }
            }
        }

        // If the grid cluster sits directly below a `Figure X` caption (and has no `Table X` caption),
        // it is a vector block diagram or schematic figure, not a table.
        if caption.is_none() && preceded_by_figure_caption {
            continue;
        }

        regions.push(CandidateTableRegion {
            bbox,
            h_rules,
            v_rules,
            caption,
            caption_bbox,
        });
    }

    regions
}

fn is_major_heading_line(line: &TextLine) -> bool {
    static RE_SEC: OnceLock<Regex> = OnceLock::new();
    let re = RE_SEC.get_or_init(|| Regex::new(r"^\d+(?:\.\d+)+\s+[A-Z]").expect("valid regex"));
    line.is_bold && line.font_size >= 10.5 && re.is_match(line.text.trim())
}

fn build_table_from_region(
    page: &mut RawPageData,
    _lines: &[TextLine],
    cand: CandidateTableRegion,
    ordinal: usize,
) -> Option<ExtractedTable> {
    // Gather all unconsumed spans inside the table bounding box
    let table_spans: Vec<TextSpan> = page
        .spans
        .iter()
        .filter(|s| !s.consumed && cand.bbox.contains_center_of(&s.bbox, 3.0))
        .cloned()
        .collect();

    if table_spans.len() < 2 {
        return None;
    }

    // Ignore visual 32-bit/16-bit register bit-box diagrams ("31 30 29 28 ...") so they are handled
    // by `suppress_visual_register_bitboxes` instead of being emitted as noisy 16-column tables.
    let internal_lines = assemble_text_lines(&table_spans, 2.2);
    if looks_like_visual_register_bitbox(&internal_lines, &cand.caption) {
        return None;
    }

    // Determine row intervals `[y_top, y_bottom]` from horizontal rules
    let row_intervals = build_row_intervals(&cand.h_rules, &internal_lines);
    if row_intervals.len() < 2 {
        return None;
    }

    // Determine column intervals `[x_left, x_right]` from vertical rules + span alignment
    let col_intervals =
        build_column_intervals(&cand.bbox, &cand.v_rules, &table_spans, &row_intervals);
    if col_intervals.len() < 2 {
        return None;
    }

    let num_cols = col_intervals.len();
    let mut raw_rows: Vec<Vec<String>> = Vec::with_capacity(row_intervals.len());

    for &(ry0, ry1) in &row_intervals {
        let row_spans: Vec<TextSpan> = table_spans
            .iter()
            .filter(|s| {
                let cy = s.bbox.center_y();
                cy >= ry0 - 1.5 && cy <= ry1 + 1.5
            })
            .cloned()
            .collect();

        if row_spans.is_empty() {
            continue;
        }

        let mut row_cells = vec![String::new(); num_cols];
        for (c_idx, &(cx0, cx1)) in col_intervals.iter().enumerate() {
            let cell_spans: Vec<TextSpan> = row_spans
                .iter()
                .filter(|s| {
                    let cx = s.bbox.center_x();
                    cx >= cx0 - 2.0 && cx <= cx1 + 2.0
                })
                .cloned()
                .collect();

            if !cell_spans.is_empty() {
                row_cells[c_idx] = format_cell_text(&cell_spans);
            }
        }

        if row_cells.iter().any(|c| !c.trim().is_empty()) {
            raw_rows.push(row_cells);
        }
    }

    if raw_rows.len() < 2 {
        return None;
    }

    // Flatten multi-level headers if the top 2 rows are both header rows (e.g., "-40°C" over "TYP | MAX")
    let (mut headers, mut body_rows) = split_and_flatten_headers(raw_rows);
    if headers.is_empty() || body_rows.is_empty() {
        return None;
    }

    // Strip pure navigation link columns (such as TI's `Section: Go` column)
    strip_navigation_columns(&mut headers, &mut body_rows);

    // Drop completely empty columns
    drop_empty_columns(&mut headers, &mut body_rows);

    if headers.len() < 2 || body_rows.is_empty() {
        return None;
    }

    // Mark spans inside this table and its caption as consumed
    for sp in page.spans.iter_mut() {
        if cand.bbox.contains_center_of(&sp.bbox, 3.0) {
            sp.consumed = true;
        }
        if let Some(cap_box) = &cand.caption_bbox {
            if cap_box.contains_center_of(&sp.bbox, 2.5) {
                sp.consumed = true;
            }
        }
    }

    let id = extract_table_id(cand.caption.as_deref(), page.page_number, ordinal);

    Some(ExtractedTable {
        id,
        caption: cand.caption,
        page_start: page.page_number,
        page_end: page.page_number,
        bbox: cand.bbox,
        headers,
        rows: body_rows,
        section_file: String::new(),
        line_number: 0,
    })
}

fn looks_like_visual_register_bitbox(lines: &[TextLine], caption: &Option<String>) -> bool {
    if let Some(cap) = caption {
        if cap.trim().starts_with("Table ") {
            return false;
        }
    }
    static RE_BITS: OnceLock<Regex> = OnceLock::new();
    let re = RE_BITS.get_or_init(|| {
        Regex::new(r"\b(?:31\s+30\s+29\s+28|15\s+14\s+13\s+12|7\s+6\s+5\s+4\s+3\s+2\s+1\s+0)\b")
            .expect("valid regex")
    });
    lines.iter().any(|l| re.is_match(&l.text))
}

fn build_row_intervals(h_rules: &[f32], lines: &[TextLine]) -> Vec<(f32, f32)> {
    let mut intervals = Vec::new();
    for w in h_rules.windows(2) {
        let y0 = w[0];
        let y1 = w[1];
        if (y1 - y0) >= 5.0 {
            intervals.push((y0, y1));
        }
    }

    // If a table only has top, header-bottom, and bottom rules (<= 3 intervals) but many text lines,
    // subdivide large intervals that don't have internal horizontal rules when every line starts at left edge.
    if intervals.len() <= 2 && lines.len() >= 4 {
        let mut refined = Vec::new();
        for (y0, y1) in intervals {
            let band_lines: Vec<&TextLine> = lines
                .iter()
                .filter(|l| l.bbox.center_y() >= y0 && l.bbox.center_y() <= y1)
                .collect();
            if band_lines.len() >= 3 && (y1 - y0) > 36.0 {
                for (i, bl) in band_lines.iter().enumerate() {
                    let top = if i == 0 {
                        y0
                    } else {
                        (band_lines[i - 1].bbox.y1 + bl.bbox.y0) * 0.5
                    };
                    let bot = if i + 1 == band_lines.len() {
                        y1
                    } else {
                        (bl.bbox.y1 + band_lines[i + 1].bbox.y0) * 0.5
                    };
                    refined.push((top, bot));
                }
            } else {
                refined.push((y0, y1));
            }
        }
        return refined;
    }

    intervals
}

fn build_column_intervals(
    table_bbox: &BBox,
    v_rules: &[f32],
    spans: &[TextSpan],
    row_intervals: &[(f32, f32)],
) -> Vec<(f32, f32)> {
    // Filter vertical rules to those inside or bounding the table
    let mut x_bounds: Vec<f32> = v_rules
        .iter()
        .copied()
        .filter(|&x| x >= table_bbox.x0 - 4.0 && x <= table_bbox.x1 + 4.0)
        .collect();

    if x_bounds
        .first()
        .is_none_or(|&x| (x - table_bbox.x0).abs() > 8.0)
    {
        x_bounds.insert(0, table_bbox.x0);
    }
    if x_bounds
        .last()
        .is_none_or(|&x| (x - table_bbox.x1).abs() > 8.0)
    {
        x_bounds.push(table_bbox.x1);
    }
    x_bounds = dedup_coords(x_bounds, 6.0);

    // If vertical rules already divide the table into >= 3 columns, check whether any wide column
    // contains multiple distinct header/body columns without vertical lines.
    if x_bounds.len() >= 4 {
        return x_bounds
            .windows(2)
            .filter(|w| (w[1] - w[0]) >= 10.0)
            .map(|w| (w[0], w[1]))
            .collect();
    }

    // Fallback for tables without vertical rules: cluster horizontal span intervals across rows
    let mut intervals: Vec<(f32, f32)> = Vec::new();
    for &(ry0, ry1) in row_intervals.iter().take(15) {
        let mut row_spans: Vec<&TextSpan> = spans
            .iter()
            .filter(|s| {
                let cy = s.bbox.center_y();
                cy >= ry0 - 1.0 && cy <= ry1 + 1.0
            })
            .collect();
        row_spans.sort_by(|a, b| {
            a.bbox
                .x0
                .partial_cmp(&b.bbox.x0)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Merge spans on the same line that are separated by normal word spaces (< 8 pt)
        let mut row_phrases: Vec<(f32, f32)> = Vec::new();
        for sp in row_spans {
            if let Some(last) = row_phrases.last_mut() {
                if sp.bbox.x0 - last.1 <= 7.5 {
                    last.1 = last.1.max(sp.bbox.x1);
                    continue;
                }
            }
            row_phrases.push((sp.bbox.x0, sp.bbox.x1));
        }
        // Only use rows that have >= 2 distinct horizontal phrases (ignore full-width banner rows)
        if row_phrases.len() >= 2 {
            intervals.extend(row_phrases);
        }
    }

    if intervals.is_empty() {
        return Vec::new();
    }

    // Merge overlapping X-intervals across rows to discover natural column tracks
    intervals.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
    let mut col_tracks: Vec<(f32, f32)> = Vec::new();
    for (x0, x1) in intervals {
        if let Some(last) = col_tracks.last_mut() {
            if x0 <= last.1 + 4.0 {
                last.1 = last.1.max(x1);
                continue;
            }
        }
        col_tracks.push((x0, x1));
    }

    if col_tracks.len() < 2 {
        return Vec::new();
    }

    let mut result = Vec::with_capacity(col_tracks.len());
    for i in 0..col_tracks.len() {
        let left = if i == 0 {
            table_bbox.x0
        } else {
            (col_tracks[i - 1].1 + col_tracks[i].0) * 0.5
        };
        let right = if i + 1 == col_tracks.len() {
            table_bbox.x1
        } else {
            (col_tracks[i].1 + col_tracks[i + 1].0) * 0.5
        };
        result.push((left, right));
    }
    result
}

/// Joins spans inside a single table cell, repairing mid-identifier line wraps (`GEN_EVENT\n0` -> `GEN_EVENT0`).
fn format_cell_text(cell_spans: &[TextSpan]) -> String {
    let lines = assemble_text_lines(cell_spans, 2.2);
    if lines.is_empty() {
        return String::new();
    }
    if lines.len() == 1 {
        return sanitize_markdown_cell(&lines[0].text);
    }

    let mut out = String::new();
    for (idx, line) in lines.iter().enumerate() {
        let t = line.text.trim();
        if t.is_empty() {
            continue;
        }
        if idx == 0 {
            out.push_str(t);
            continue;
        }

        // Check for mid-identifier wrap inside a narrow column:
        // e.g. "GEN_EVENT" + "0" -> "GEN_EVENT0", or "SYSCTL_" + "C1105" -> "SYSCTL_C1105"
        if should_join_without_space(&out, t) {
            out.push_str(t);
        } else if is_enum_value_line(t) {
            out.push_str("; ");
            out.push_str(t);
        } else {
            out.push(' ');
            out.push_str(t);
        }
    }

    sanitize_markdown_cell(&out)
}

fn should_join_without_space(prev: &str, next: &str) -> bool {
    let prev_trim = prev.trim();
    let next_trim = next.trim();
    if prev_trim.is_empty() || next_trim.is_empty() {
        return false;
    }
    // Case 1: prev ends with '_' or '-' (e.g. "GEN_EVENT_" + "0" or "5V-" + "tol")
    if prev_trim.ends_with('_') || prev_trim.ends_with('-') {
        return true;
    }
    // Case 2: prev is a single uppercase identifier token containing '_' (like "GEN_EVENT")
    // and next is a short digit or uppercase suffix ("0", "1")
    let prev_is_ident = !prev_trim.contains(' ')
        && prev_trim.contains('_')
        && prev_trim
            .chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_');
    let next_is_short_suffix = next_trim.len() <= 3
        && next_trim
            .chars()
            .all(|c| c.is_ascii_digit() || c.is_ascii_uppercase());

    prev_is_ident && next_is_short_suffix
}

fn is_enum_value_line(line: &str) -> bool {
    static RE_ENUM: OnceLock<Regex> = OnceLock::new();
    let re = RE_ENUM.get_or_init(|| {
        Regex::new(r"^[0-9A-Fa-f]+(?:h|b|d|-[0-9A-Fa-f]+)?\s*=").expect("valid regex")
    });
    re.is_match(line.trim())
}

fn sanitize_markdown_cell(s: &str) -> String {
    normalize_technical_text(s)
        .replace('|', "\\|")
        .replace('\n', " ")
        .trim()
        .to_string()
}

/// Detects whether row 0 and row 1 form a 2-level header (e.g. `-40°C | 25°C` above `TYP | MAX`)
/// and flattens them into a single unambiguous header row.
fn split_and_flatten_headers(mut rows: Vec<Vec<String>>) -> (Vec<String>, Vec<Vec<String>>) {
    if rows.is_empty() {
        return (Vec::new(), Vec::new());
    }
    if rows.len() == 1 {
        return (rows.remove(0), Vec::new());
    }

    let r0 = &rows[0];
    let r1 = &rows[1];

    // Check if r1 looks like sub-headers ("TYP", "MAX", "MIN", "UNIT", "VALUE")
    let subheader_keywords = ["typ", "max", "min", "nom", "unit", "test", "cond"];
    let r1_non_empty: Vec<&String> = r1.iter().filter(|c| !c.trim().is_empty()).collect();
    let r1_matches = r1_non_empty
        .iter()
        .filter(|c| {
            let low = c.to_ascii_lowercase();
            subheader_keywords.iter().any(|k| low == *k)
        })
        .count();

    if rows.len() >= 3 && r1_non_empty.len() >= 2 && r1_matches * 2 >= r1_non_empty.len() {
        // Carry forward group headers across spanned sub-columns (e.g. "-40°C" across "TYP" and "MAX")
        let mut flattened = Vec::with_capacity(r0.len());
        let mut active_group = String::new();
        for col in 0..r0.len() {
            let top = r0[col].trim();
            let sub = r1[col].trim();
            if !top.is_empty() {
                if sub.is_empty() {
                    active_group.clear();
                    flattened.push(top.to_string());
                } else {
                    active_group = top.to_string();
                    flattened.push(format!("{active_group} {sub}"));
                }
            } else if !sub.is_empty() {
                if !active_group.is_empty() {
                    flattened.push(format!("{active_group} {sub}"));
                } else {
                    flattened.push(sub.to_string());
                }
            } else {
                flattened.push(String::new());
            }
        }
        let body = rows.split_off(2);
        return (flattened, body);
    }

    let headers = rows.remove(0);
    (headers, rows)
}

/// Removes pure PDF navigation link columns (such as `Section` where every cell is `"Go"`).
fn strip_navigation_columns(headers: &mut Vec<String>, rows: &mut [Vec<String>]) {
    if headers.is_empty() || rows.is_empty() {
        return;
    }

    let mut col = headers.len();
    while col > 0 {
        col -= 1;
        let h_low = headers[col].trim().to_ascii_lowercase();
        let non_empty: Vec<&str> = rows
            .iter()
            .filter_map(|r| r.get(col).map(|s| s.trim()))
            .filter(|s| !s.is_empty())
            .collect();

        if !non_empty.is_empty() {
            let go_count = non_empty
                .iter()
                .filter(|s| s.eq_ignore_ascii_case("go"))
                .count();
            if (h_low == "section" || h_low.is_empty()) && go_count * 5 >= non_empty.len() * 4 {
                headers.remove(col);
                for r in rows.iter_mut() {
                    if col < r.len() {
                        r.remove(col);
                    }
                }
            }
        }
    }
}

/// Removes columns that are completely empty in both the header and all body rows.
fn drop_empty_columns(headers: &mut Vec<String>, rows: &mut [Vec<String>]) {
    let mut col = headers.len();
    while col > 0 {
        col -= 1;
        let header_empty = headers[col].trim().is_empty();
        let all_rows_empty = rows
            .iter()
            .all(|r| r.get(col).is_none_or(|c| c.trim().is_empty()));

        if header_empty && all_rows_empty {
            headers.remove(col);
            for r in rows.iter_mut() {
                if col < r.len() {
                    r.remove(col);
                }
            }
        } else if header_empty {
            headers[col] = format!("Col {}", col + 1);
        }
    }
}

fn extract_table_id(caption: Option<&str>, page_number: usize, ordinal: usize) -> String {
    static RE_ID: OnceLock<Regex> = OnceLock::new();
    let re = RE_ID.get_or_init(|| Regex::new(r"^Table\s+(\d+(?:[-\.]\d+)*)").expect("valid regex"));
    if let Some(cap) = caption {
        if let Some(caps) = re.captures(cap.trim()) {
            if let Some(m) = caps.get(1) {
                return format!("table-{}", m.as_str().replace('.', "-"));
            }
        }
    }
    format!("p{page_number:03}-table-{ordinal}")
}

fn dedup_coords(mut coords: Vec<f32>, tol: f32) -> Vec<f32> {
    if coords.is_empty() {
        return coords;
    }
    coords.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut out = vec![coords[0]];
    for c in coords.into_iter().skip(1) {
        if let Some(last) = out.last_mut() {
            if (c - *last).abs() <= tol {
                *last = (*last + c) * 0.5;
                continue;
            }
        }
        out.push(c);
    }
    out
}

/// Renders an `ExtractedTable` into GitHub-Flavored Markdown lines.
pub fn render_table_markdown(table: &ExtractedTable) -> Vec<String> {
    let mut lines = Vec::with_capacity(table.rows.len() + 4);
    if let Some(cap) = &table.caption {
        let page_cite = if table.page_start == table.page_end {
            format!("p. {}", table.page_start)
        } else {
            format!("pp. {}-{}", table.page_start, table.page_end)
        };
        lines.push(format!("**{} ({})**", cap.trim(), page_cite));
        lines.push(String::new());
    }

    let num_cols = table.headers.len();
    lines.push(format!("| {} |", table.headers.join(" | ")));
    lines.push(format!("| {} |", vec!["---"; num_cols].join(" | ")));

    for row in &table.rows {
        let mut padded = Vec::with_capacity(num_cols);
        for c in 0..num_cols {
            padded.push(row.get(c).map(|s| s.as_str()).unwrap_or(""));
        }
        lines.push(format!("| {} |", padded.join(" | ")));
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mid_identifier_wrap_join() {
        let spans = vec![
            TextSpan {
                text: "GEN_EVENT".to_string(),
                bbox: BBox::new(10.0, 10.0, 60.0, 20.0),
                font_size: 9.0,
                is_bold: false,
                is_mono: false,
                consumed: false,
            },
            TextSpan {
                text: "0".to_string(),
                bbox: BBox::new(10.0, 22.0, 18.0, 32.0),
                font_size: 9.0,
                is_bold: false,
                is_mono: false,
                consumed: false,
            },
        ];
        assert_eq!(format_cell_text(&spans), "GEN_EVENT0");
    }

    #[test]
    fn test_two_level_header_flattening() {
        let rows = vec![
            vec![
                "PARAMETER".to_string(),
                "-40°C".to_string(),
                "".to_string(),
                "25°C".to_string(),
                "".to_string(),
                "UNIT".to_string(),
            ],
            vec![
                "".to_string(),
                "TYP".to_string(),
                "MAX".to_string(),
                "TYP".to_string(),
                "MAX".to_string(),
                "".to_string(),
            ],
            vec![
                "IDDRUN".to_string(),
                "2.9".to_string(),
                "2.9".to_string(),
                "2.9".to_string(),
                "2.9".to_string(),
                "mA".to_string(),
            ],
        ];
        let (headers, body) = split_and_flatten_headers(rows);
        assert_eq!(
            headers,
            vec![
                "PARAMETER",
                "-40°C TYP",
                "-40°C MAX",
                "25°C TYP",
                "25°C MAX",
                "UNIT"
            ]
        );
        assert_eq!(body.len(), 1);
    }

    #[test]
    fn test_strip_navigation_go_column() {
        let mut headers = vec![
            "Offset".to_string(),
            "Acronym".to_string(),
            "Section".to_string(),
        ];
        let mut rows = vec![
            vec!["400h".to_string(), "FSUB_0".to_string(), "Go".to_string()],
            vec!["404h".to_string(), "FSUB_1".to_string(), "Go".to_string()],
        ];
        strip_navigation_columns(&mut headers, &mut rows);
        assert_eq!(headers, vec!["Offset", "Acronym"]);
        assert_eq!(rows[0], vec!["400h", "FSUB_0"]);
    }
}
