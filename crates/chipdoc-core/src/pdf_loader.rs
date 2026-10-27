use crate::cleaner::{clean_metadata_text, slugify};
use crate::error::{ChipdocError, Result};
use crate::model::{
    BBox, ConvertConfig, DocMetadata, FigureRegion, GridSegment, RawPageData, TextSpan, TocEntry,
};
use image::codecs::png::{CompressionType, FilterType, PngEncoder};
use image::{DynamicImage, ImageEncoder};
use pdfium_render::prelude::*;
use rayon::prelude::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufWriter, Read, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Computes a fast, deterministic 64-bit FNV-1a fingerprint and byte size for a PDF file
/// by hashing up to three 128 KiB windows (head, middle, tail) plus the exact file length.
/// Completes in <0.2ms even on 100 MB Technical Reference Manuals.
pub fn compute_pdf_fingerprint(input_pdf: &Path) -> Result<(String, u64)> {
    if !input_pdf.exists() {
        return Err(ChipdocError::PdfNotFound(input_pdf.to_path_buf()));
    }
    let mut file = File::open(input_pdf).map_err(|source| ChipdocError::Io {
        path: input_pdf.to_path_buf(),
        source,
    })?;
    let len = file
        .metadata()
        .map_err(|source| ChipdocError::Io {
            path: input_pdf.to_path_buf(),
            source,
        })?
        .len();

    const FNV_OFFSET: u64 = 0xcbf29ce484222325;
    const FNV_PRIME: u64 = 0x00000100000001b3;
    const WINDOW: usize = 128 * 1024;

    let mut hash = FNV_OFFSET;
    for b in len.to_le_bytes() {
        hash ^= b as u64;
        hash = hash.wrapping_mul(FNV_PRIME);
    }

    let mut buf = vec![0u8; WINDOW];
    let offsets: Vec<u64> = if len <= (WINDOW as u64) * 3 {
        vec![0]
    } else {
        vec![0, len / 2 - (WINDOW as u64) / 2, len - (WINDOW as u64)]
    };

    for off in offsets {
        if file.seek(SeekFrom::Start(off)).is_ok() {
            let n = file.read(&mut buf).unwrap_or(0);
            for &b in &buf[..n] {
                hash ^= b as u64;
                hash = hash.wrapping_mul(FNV_PRIME);
            }
        }
    }

    Ok((format!("{hash:016x}"), len))
}

/// Binds and initializes the PDFium library from configured or discovered paths.
pub fn bind_pdfium(explicit_dir: Option<&Path>) -> Result<Pdfium> {
    let mut candidate_dirs: Vec<PathBuf> = Vec::new();

    if let Some(dir) = explicit_dir {
        candidate_dirs.push(dir.to_path_buf());
    }
    if let Ok(env_dir) = std::env::var("PDFIUM_DYNAMIC_LIB_PATH") {
        candidate_dirs.push(PathBuf::from(env_dir));
    }
    if let Ok(env_dir) = std::env::var("PDFIUM_LIB_PATH") {
        candidate_dirs.push(PathBuf::from(env_dir));
    }
    if let Ok(cwd) = std::env::current_dir() {
        candidate_dirs.push(cwd.join("bin"));
        candidate_dirs.push(cwd);
    }
    if let Ok(exe) = std::env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            candidate_dirs.push(exe_dir.to_path_buf());
            candidate_dirs.push(exe_dir.join("bin"));
            if let Some(p1) = exe_dir.parent() {
                candidate_dirs.push(p1.join("bin"));
                if let Some(p2) = p1.parent() {
                    candidate_dirs.push(p2.join("bin"));
                }
            }
        }
    }
    candidate_dirs.push(PathBuf::from(
        r"D:\projects\docling\venv\Lib\site-packages\pypdfium2_raw",
    ));

    for dir in &candidate_dirs {
        let lib_path = Pdfium::pdfium_platform_library_name_at_path(dir);
        if lib_path.exists() {
            if let Ok(bindings) = Pdfium::bind_to_library(&lib_path) {
                return Ok(Pdfium::new(bindings));
            }
        }
    }

    let bindings =
        Pdfium::bind_to_system_library().map_err(|e| ChipdocError::PdfiumBind(format!("{e:?}")))?;
    Ok(Pdfium::new(bindings))
}

/// Extracts document metadata, TOC bookmarks, and raw geometric/text data for the requested pages.
pub fn load_pdf_document<'a>(
    pdfium: &'a Pdfium,
    input_pdf: &Path,
    page_filter: Option<&[usize]>,
) -> Result<(
    PdfDocument<'a>,
    DocMetadata,
    Vec<TocEntry>,
    Vec<RawPageData>,
)> {
    if !input_pdf.exists() {
        return Err(ChipdocError::PdfNotFound(input_pdf.to_path_buf()));
    }

    let doc = pdfium
        .load_pdf_from_file(input_pdf, None)
        .map_err(|e| ChipdocError::Pdfium {
            path: input_pdf.display().to_string(),
            message: format!("{e:?}"),
        })?;

    let total_pages = doc.pages().len() as usize;
    let file_name = input_pdf
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("document.pdf")
        .to_string();
    let doc_stem = input_pdf
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("document")
        .to_string();

    let meta_tags = doc.metadata();
    let get_tag = |tag_type: PdfDocumentMetadataTagType| -> Option<String> {
        meta_tags
            .get(tag_type)
            .map(|t| clean_metadata_text(t.value()))
            .filter(|s| !s.is_empty())
    };

    let page_numbers: Vec<usize> = match page_filter {
        Some(filter) if !filter.is_empty() => filter
            .iter()
            .copied()
            .filter(|&p| p >= 1 && p <= total_pages)
            .collect(),
        _ => (1..=total_pages).collect(),
    };

    let (source_hash, source_size_bytes) = compute_pdf_fingerprint(input_pdf)?;

    let metadata = DocMetadata {
        file_name,
        doc_stem,
        source_hash,
        source_size_bytes,
        title: get_tag(PdfDocumentMetadataTagType::Title),
        author: get_tag(PdfDocumentMetadataTagType::Author),
        subject: get_tag(PdfDocumentMetadataTagType::Subject),
        keywords: get_tag(PdfDocumentMetadataTagType::Keywords),
        creator: get_tag(PdfDocumentMetadataTagType::Creator),
        producer: get_tag(PdfDocumentMetadataTagType::Producer),
        total_pages,
        converted_pages: page_numbers.len(),
    };

    let toc = extract_bookmarks(&doc, total_pages);

    let mut raw_pages = Vec::with_capacity(page_numbers.len());
    for &page_num in &page_numbers {
        let page_idx = (page_num - 1) as u16;
        let page = doc
            .pages()
            .get(page_idx)
            .map_err(|e| ChipdocError::Pdfium {
                path: input_pdf.display().to_string(),
                message: format!("page {page_num}: {e:?}"),
            })?;
        raw_pages.push(extract_single_page(&page, page_num));
    }

    Ok((doc, metadata, toc, raw_pages))
}

fn is_caption_bookmark(title: &str) -> bool {
    static RE_CAPTION_BM: OnceLock<Regex> = OnceLock::new();
    let re = RE_CAPTION_BM.get_or_init(|| {
        Regex::new(r"^(?i)(?:table|figure|fig\.)\s+\d+(?:[-\.]\d+)*(?:\b|[\.:])")
            .expect("valid regex")
    });
    re.is_match(title.trim())
}

fn extract_bookmarks(doc: &PdfDocument<'_>, total_pages: usize) -> Vec<TocEntry> {
    let mut entries: Vec<TocEntry> = Vec::new();

    for bookmark in doc.bookmarks().iter() {
        let title = clean_metadata_text(bookmark.title().unwrap_or_default().trim());
        if title.is_empty() || is_caption_bookmark(&title) {
            continue;
        }
        let start_page = bookmark
            .destination()
            .and_then(|d| d.page_index().ok())
            .or_else(|| {
                bookmark.action().and_then(|a| {
                    a.as_local_destination_action()
                        .and_then(|la| la.destination().ok())
                        .and_then(|d| d.page_index().ok())
                })
            })
            .map(|p| (p as usize) + 1)
            .unwrap_or(1)
            .clamp(1, total_pages.max(1));

        // Infer hierarchy level from section numbering ("1.2.3 Title" -> level 2) or bookmark parent chain
        let level = infer_bookmark_level(&bookmark, &title);
        let toc_idx = entries.len() + 1;

        entries.push(TocEntry {
            id: format!("toc-{toc_idx}"),
            level,
            slug: slugify(&title),
            title,
            start_page,
            end_page: total_pages,
        });
    }

    // Populate end_page for each bookmark based on the next bookmark at the same or higher level
    for i in 0..entries.len() {
        let cur_lvl = entries[i].level;
        let cur_start = entries[i].start_page;
        let mut end_page = total_pages;
        for next in entries.iter().skip(i + 1) {
            if next.level <= cur_lvl {
                end_page = if next.start_page > cur_start {
                    next.start_page - 1
                } else {
                    cur_start
                };
                break;
            }
        }
        entries[i].end_page = end_page.clamp(cur_start, total_pages.max(1));
    }

    entries
}

fn infer_bookmark_level(bookmark: &PdfBookmark<'_>, title: &str) -> usize {
    // First check explicit numeric section prefix ("1" -> 0, "1.1" -> 1, "1.1.1" -> 2)
    if let Some(first_tok) = title.split_whitespace().next() {
        let clean = first_tok.trim_end_matches('.');
        if !clean.is_empty()
            && clean.chars().all(|c| c.is_ascii_digit() || c == '.')
            && clean.chars().next().is_some_and(|c| c.is_ascii_digit())
        {
            return clean.chars().filter(|&c| c == '.').count();
        }
    }

    // Otherwise count parent hops if available
    let mut level = 0usize;
    let mut current_parent = bookmark.parent();
    while let Some(parent) = current_parent {
        level += 1;
        if level >= 6 {
            break;
        }
        current_parent = parent.parent();
    }
    level
}

fn extract_single_page(page: &PdfPage<'_>, page_number: usize) -> RawPageData {
    let width = page.width().value;
    let height = page.height().value;
    let text_page = page.text().ok();

    let mut spans: Vec<TextSpan> = Vec::new();
    let mut grid_segments: Vec<GridSegment> = Vec::new();
    let mut graphic_bboxes: Vec<BBox> = Vec::new();
    let mut image_bboxes: Vec<BBox> = Vec::new();

    for obj in page.objects().iter() {
        let Ok(quad) = obj.bounds() else {
            continue;
        };
        let rect = quad.to_rect();
        // Convert from PDF bottom-up coordinates to top-down coordinates
        let y0 = (height - rect.top().value).clamp(0.0, height);
        let y1 = (height - rect.bottom().value).clamp(0.0, height);
        let x0 = rect.left().value.clamp(0.0, width);
        let x1 = rect.right().value.clamp(0.0, width);
        let bbox = BBox::new(x0, y0, x1, y1);

        match &obj {
            PdfPageObject::Text(text_obj) => {
                if bbox.width() <= 0.1 || bbox.height() <= 0.1 {
                    continue;
                }
                let raw_text = match &text_page {
                    Some(tp) => tp.for_object(text_obj),
                    None => text_obj.text(),
                };
                if raw_text.trim().is_empty() {
                    continue;
                }
                let font_size = text_obj.scaled_font_size().value.max(1.0);
                let lead_spaces = raw_text
                    .chars()
                    .take_while(|&c| c == ' ' || c == '\t')
                    .count();
                let trail_spaces = raw_text
                    .chars()
                    .rev()
                    .take_while(|&c| c == ' ' || c == '\t')
                    .count();
                let mut adj_bbox = bbox;
                if lead_spaces > 0 || trail_spaces > 0 {
                    let new_x0 = adj_bbox.x0 + (font_size * 0.22) * (lead_spaces as f32);
                    let new_x1 = adj_bbox.x1 - (font_size * 0.22) * (trail_spaces as f32);
                    if new_x0 + 0.5 < new_x1 {
                        adj_bbox.x0 = new_x0;
                        adj_bbox.x1 = new_x1;
                    }
                }
                let font = text_obj.font();
                let font_name = font.name().to_ascii_lowercase();
                let weight_bold = matches!(
                    font.weight(),
                    Ok(PdfFontWeight::Weight600
                        | PdfFontWeight::Weight700Bold
                        | PdfFontWeight::Weight800
                        | PdfFontWeight::Weight900)
                );
                let is_bold = weight_bold
                    || font.is_bold_reenforced()
                    || font_name.contains("bold")
                    || font_name.ends_with("bd")
                    || font_name.contains("-bd")
                    || font_name.contains("heavy")
                    || font_name.contains("black");
                let is_mono = font.is_fixed_pitch()
                    || font_name.contains("mono")
                    || font_name.contains("cour")
                    || font_name.contains("consol");

                spans.push(TextSpan {
                    text: raw_text.trim().to_string(),
                    bbox: adj_bbox,
                    font_size,
                    is_bold,
                    is_mono,
                    consumed: false,
                });
            }
            _ => {
                extract_object_geometry(
                    &obj,
                    bbox,
                    width,
                    height,
                    0,
                    &mut grid_segments,
                    &mut graphic_bboxes,
                    &mut image_bboxes,
                );
            }
        }
    }

    RawPageData {
        page_number,
        width,
        height,
        spans,
        grid_segments,
        graphic_bboxes,
        image_bboxes,
    }
}

#[allow(clippy::too_many_arguments)]
fn extract_object_geometry(
    obj: &PdfPageObject<'_>,
    bbox: BBox,
    page_w: f32,
    page_h: f32,
    depth: usize,
    grid_segments: &mut Vec<GridSegment>,
    graphic_bboxes: &mut Vec<BBox>,
    image_bboxes: &mut Vec<BBox>,
) {
    if depth > 4 {
        return;
    }
    match obj {
        PdfPageObject::Path(path_obj) => {
            extract_path_geometry(
                path_obj,
                bbox,
                page_w,
                page_h,
                grid_segments,
                graphic_bboxes,
            );
        }
        PdfPageObject::Image(_) => {
            if bbox.width() >= 8.0 && bbox.height() >= 8.0 {
                image_bboxes.push(bbox);
            }
        }
        PdfPageObject::XObjectForm(form_obj) => {
            if bbox.width() >= 24.0 && bbox.height() >= 20.0 && bbox.area() < page_w * page_h * 0.85
            {
                graphic_bboxes.push(bbox);
            }
            for child in form_obj.iter() {
                if let Ok(c_quad) = child.bounds() {
                    let cr = c_quad.to_rect();
                    let cy0 = (page_h - cr.top().value).clamp(0.0, page_h);
                    let cy1 = (page_h - cr.bottom().value).clamp(0.0, page_h);
                    let cx0 = cr.left().value.clamp(0.0, page_w);
                    let cx1 = cr.right().value.clamp(0.0, page_w);
                    let cb = BBox::new(cx0, cy0, cx1, cy1);
                    extract_object_geometry(
                        &child,
                        cb,
                        page_w,
                        page_h,
                        depth + 1,
                        grid_segments,
                        graphic_bboxes,
                        image_bboxes,
                    );
                }
            }
        }
        _ => {}
    }
}

fn extract_path_geometry(
    path_obj: &PdfPagePathObject<'_>,
    bbox: BBox,
    page_w: f32,
    page_h: f32,
    grid_segments: &mut Vec<GridSegment>,
    graphic_bboxes: &mut Vec<BBox>,
) {
    let w = bbox.width();
    let h = bbox.height();

    // 1. Thin horizontal rule or stroked horizontal line
    if w >= 12.0 && h <= 4.5 {
        let mid_y = bbox.center_y();
        grid_segments.push(GridSegment {
            x0: bbox.x0,
            y0: mid_y,
            x1: bbox.x1,
            y1: mid_y,
            is_horizontal: true,
        });
        if w <= page_w * 0.65 {
            graphic_bboxes.push(bbox);
        }
        return;
    }

    // 2. Thin vertical rule or stroked vertical line
    if h >= 8.0 && w <= 4.5 {
        let mid_x = bbox.center_x();
        grid_segments.push(GridSegment {
            x0: mid_x,
            y0: bbox.y0,
            x1: mid_x,
            y1: bbox.y1,
            is_horizontal: false,
        });
        if w <= page_w * 0.65 {
            graphic_bboxes.push(bbox);
        }
        return;
    }

    // 3. Inspect internal path segments for rectangles and multi-line table grids
    let mut prev_pt: Option<(f32, f32)> = None;
    let mut start_pt: Option<(f32, f32)> = None;
    let mut axis_segments = 0usize;
    let mut diagonal_segments = 0usize;

    let segs = match path_obj.matrix() {
        Ok(m) => path_obj.segments().transform(m),
        Err(_) => path_obj.segments(),
    };

    for seg in segs.iter() {
        let (pt_x, pt_y) = seg.point();
        let sx = pt_x.value.clamp(0.0, page_w);
        let sy = (page_h - pt_y.value).clamp(0.0, page_h);

        match seg.segment_type() {
            PdfPathSegmentType::MoveTo => {
                prev_pt = Some((sx, sy));
                start_pt = Some((sx, sy));
            }
            PdfPathSegmentType::LineTo => {
                if let Some((px, py)) = prev_pt {
                    let dx = (sx - px).abs();
                    let dy = (sy - py).abs();
                    if dy <= 1.5 && dx >= 12.0 {
                        axis_segments += 1;
                        grid_segments.push(GridSegment {
                            x0: px.min(sx),
                            y0: (py + sy) * 0.5,
                            x1: px.max(sx),
                            y1: (py + sy) * 0.5,
                            is_horizontal: true,
                        });
                    } else if dx <= 1.5 && dy >= 8.0 {
                        axis_segments += 1;
                        grid_segments.push(GridSegment {
                            x0: (px + sx) * 0.5,
                            y0: py.min(sy),
                            x1: (px + sx) * 0.5,
                            y1: py.max(sy),
                            is_horizontal: false,
                        });
                    } else if dx > 2.0 && dy > 2.0 {
                        diagonal_segments += 1;
                    }
                }
                prev_pt = Some((sx, sy));
            }
            PdfPathSegmentType::BezierTo => {
                diagonal_segments += 1;
                prev_pt = Some((sx, sy));
            }
            PdfPathSegmentType::Unknown => {}
        }

        if seg.is_close() {
            if let (Some((px, py)), Some((ox, oy))) = (prev_pt, start_pt) {
                let dx = (ox - px).abs();
                let dy = (oy - py).abs();
                if dy <= 1.5 && dx >= 12.0 {
                    axis_segments += 1;
                    grid_segments.push(GridSegment {
                        x0: px.min(ox),
                        y0: (py + oy) * 0.5,
                        x1: px.max(ox),
                        y1: (py + oy) * 0.5,
                        is_horizontal: true,
                    });
                } else if dx <= 1.5 && dy >= 8.0 {
                    axis_segments += 1;
                    grid_segments.push(GridSegment {
                        x0: (px + ox) * 0.5,
                        y0: py.min(oy),
                        x1: (px + ox) * 0.5,
                        y1: py.max(oy),
                        is_horizontal: false,
                    });
                }
            }
        }
    }

    // Record general graphic bounding box for figure detection
    if (w >= 4.0 || h >= 4.0) && (diagonal_segments > 0 || axis_segments > 0) {
        graphic_bboxes.push(bbox);
    }
}

/// Renders cropped PNG images for all detected `FigureRegion`s across the document.
pub fn render_detected_figures(
    doc: &PdfDocument<'_>,
    config: &ConvertConfig,
    doc_stem: &str,
    figures: &[FigureRegion],
) -> Result<usize> {
    if !config.render_figures || config.lazy_figures || figures.is_empty() {
        return Ok(0);
    }

    let bundle_dir = config.output_root.join(doc_stem);

    // Group figures by 1-based page_number so each page with figures is rendered only once
    let mut by_page: std::collections::BTreeMap<usize, Vec<&FigureRegion>> =
        std::collections::BTreeMap::new();
    for fig in figures {
        if fig.image_rel_path.is_some() {
            by_page.entry(fig.page_number).or_default().push(fig);
        }
    }

    let scale = (config.figure_dpi / 72.0).clamp(1.0, 4.5);
    let mut crops_to_encode: Vec<(PathBuf, DynamicImage)> = Vec::new();

    for (page_num, page_figs) in by_page {
        let Ok(page) = doc.pages().get((page_num - 1) as u16) else {
            continue;
        };
        let page_w = page.width().value.max(1.0);
        let page_h = page.height().value.max(1.0);
        let target_px_w = ((page_w * scale).round() as i32).max(200);

        let render_cfg = PdfRenderConfig::new()
            .set_target_width(target_px_w)
            .render_form_data(true);

        let Ok(bitmap) = page.render_with_config(&render_cfg) else {
            continue;
        };
        let dyn_img = bitmap.as_image();
        let img_w = dyn_img.width();
        let img_h = dyn_img.height();

        for fig in page_figs {
            let Some(rel_path) = &fig.image_rel_path else {
                continue;
            };
            let out_path = bundle_dir.join(rel_path);
            let rx0 = ((fig.bbox.x0 / page_w) * (img_w as f32)).floor().max(0.0) as u32;
            let ry0 = ((fig.bbox.y0 / page_h) * (img_h as f32)).floor().max(0.0) as u32;
            let rx1 = ((fig.bbox.x1 / page_w) * (img_w as f32))
                .ceil()
                .min(img_w as f32) as u32;
            let ry1 = ((fig.bbox.y1 / page_h) * (img_h as f32))
                .ceil()
                .min(img_h as f32) as u32;

            let crop_w = rx1.saturating_sub(rx0);
            let crop_h = ry1.saturating_sub(ry0);
            if crop_w < 16 || crop_h < 16 {
                continue;
            }

            let cropped = dyn_img.crop_imm(rx0, ry0, crop_w, crop_h);
            crops_to_encode.push((out_path, cropped));
        }
    }

    // Encode and write PNG files in parallel across Rayon worker threads using fast PNG compression
    crops_to_encode
        .par_iter()
        .try_for_each(|(out_path, cropped)| save_fast_png(cropped, out_path))?;

    Ok(crops_to_encode.len())
}

/// Renders and crops a single figure from `input_pdf` on demand (used when `--lazy-figures` was set).
pub fn render_single_figure_crop(
    input_pdf: &Path,
    pdfium_lib_dir: Option<&Path>,
    page_number: usize,
    bbox: BBox,
    out_path: &Path,
    dpi: f32,
) -> Result<()> {
    let pdfium = bind_pdfium(pdfium_lib_dir)?;
    let doc = pdfium
        .load_pdf_from_file(input_pdf, None)
        .map_err(|e| ChipdocError::Pdfium {
            path: input_pdf.display().to_string(),
            message: format!("{e:?}"),
        })?;
    let page = doc
        .pages()
        .get((page_number.saturating_sub(1)) as u16)
        .map_err(|e| ChipdocError::Pdfium {
            path: input_pdf.display().to_string(),
            message: format!("page {page_number}: {e:?}"),
        })?;

    let scale = (dpi / 72.0).clamp(1.0, 4.5);
    let page_w = page.width().value.max(1.0);
    let page_h = page.height().value.max(1.0);
    let target_px_w = ((page_w * scale).round() as i32).max(200);

    let render_cfg = PdfRenderConfig::new()
        .set_target_width(target_px_w)
        .render_form_data(true);

    let bitmap = page
        .render_with_config(&render_cfg)
        .map_err(|e| ChipdocError::Pdfium {
            path: input_pdf.display().to_string(),
            message: format!("render page {page_number}: {e:?}"),
        })?;
    let dyn_img = bitmap.as_image();
    let img_w = dyn_img.width();
    let img_h = dyn_img.height();

    let rx0 = ((bbox.x0 / page_w) * (img_w as f32)).floor().max(0.0) as u32;
    let ry0 = ((bbox.y0 / page_h) * (img_h as f32)).floor().max(0.0) as u32;
    let rx1 = ((bbox.x1 / page_w) * (img_w as f32))
        .ceil()
        .min(img_w as f32) as u32;
    let ry1 = ((bbox.y1 / page_h) * (img_h as f32))
        .ceil()
        .min(img_h as f32) as u32;

    let crop_w = rx1.saturating_sub(rx0).max(1);
    let crop_h = ry1.saturating_sub(ry0).max(1);
    let cropped = dyn_img.crop_imm(rx0, ry0, crop_w, crop_h);
    save_fast_png(&cropped, out_path)
}

fn save_fast_png(img: &DynamicImage, out_path: &Path) -> Result<()> {
    if let Some(parent) = out_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    let file = File::create(out_path).map_err(|source| ChipdocError::Io {
        path: out_path.to_path_buf(),
        source,
    })?;
    let writer = BufWriter::new(file);
    let encoder = PngEncoder::new_with_quality(writer, CompressionType::Fast, FilterType::Adaptive);
    let rgba = img.to_rgba8();
    encoder
        .write_image(
            rgba.as_raw(),
            rgba.width(),
            rgba.height(),
            image::ExtendedColorType::Rgba8,
        )
        .map_err(|e| ChipdocError::Image {
            path: out_path.to_path_buf(),
            message: e.to_string(),
        })
}

/// Parses a CLI page range specification like `"1-10,14,41-45"` into a sorted, deduplicated
/// vector of 1-based page numbers.
pub fn parse_page_range_expr(expr: &str) -> Result<Vec<usize>> {
    let mut pages = Vec::new();
    for part in expr.split(',') {
        let p = part.trim();
        if p.is_empty() {
            continue;
        }
        if let Some((start_s, end_s)) = p.split_once('-') {
            let start: usize = start_s
                .trim()
                .parse()
                .map_err(|_| ChipdocError::InvalidPageRange(expr.to_string()))?;
            let end: usize = end_s
                .trim()
                .parse()
                .map_err(|_| ChipdocError::InvalidPageRange(expr.to_string()))?;
            if start == 0 || end < start {
                return Err(ChipdocError::InvalidPageRange(expr.to_string()));
            }
            pages.extend(start..=end);
        } else {
            let num: usize = p
                .parse()
                .map_err(|_| ChipdocError::InvalidPageRange(expr.to_string()))?;
            if num == 0 {
                return Err(ChipdocError::InvalidPageRange(expr.to_string()));
            }
            pages.push(num);
        }
    }
    pages.sort_unstable();
    pages.dedup();
    Ok(pages)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_page_range_expr() {
        let parsed = parse_page_range_expr("1-3, 7, 14-16").expect("valid range");
        assert_eq!(parsed, vec![1, 2, 3, 7, 14, 15, 16]);
    }

    #[test]
    fn test_is_caption_bookmark() {
        assert!(is_caption_bookmark("Table 1-1. Device Summary"));
        assert!(is_caption_bookmark("Figure 3. Block diagram"));
        assert!(is_caption_bookmark("Fig. 2-4: Timing diagram"));
        assert!(!is_caption_bookmark("Table of Contents"));
        assert!(!is_caption_bookmark("1.2 Configuration Tables"));
    }
}
