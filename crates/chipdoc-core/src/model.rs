use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Bounding box in top-down page coordinates (origin `(0.0, 0.0)` at top-left of page).
/// `y0` is the top edge and `y1` is the bottom edge (`y0 <= y1`).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BBox {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl BBox {
    pub fn new(x0: f32, y0: f32, x1: f32, y1: f32) -> Self {
        Self {
            x0: x0.min(x1),
            y0: y0.min(y1),
            x1: x0.max(x1),
            y1: y0.max(y1),
        }
    }

    #[inline]
    pub fn width(&self) -> f32 {
        (self.x1 - self.x0).max(0.0)
    }

    #[inline]
    pub fn height(&self) -> f32 {
        (self.y1 - self.y0).max(0.0)
    }

    #[inline]
    pub fn area(&self) -> f32 {
        self.width() * self.height()
    }

    #[inline]
    pub fn center_x(&self) -> f32 {
        (self.x0 + self.x1) * 0.5
    }

    #[inline]
    pub fn center_y(&self) -> f32 {
        (self.y0 + self.y1) * 0.5
    }

    #[inline]
    pub fn contains_point(&self, x: f32, y: f32, tolerance: f32) -> bool {
        x >= self.x0 - tolerance
            && x <= self.x1 + tolerance
            && y >= self.y0 - tolerance
            && y <= self.y1 + tolerance
    }

    #[inline]
    pub fn contains_center_of(&self, other: &BBox, tolerance: f32) -> bool {
        self.contains_point(other.center_x(), other.center_y(), tolerance)
    }

    #[inline]
    pub fn overlaps(&self, other: &BBox, pad: f32) -> bool {
        self.x0 - pad <= other.x1
            && self.x1 + pad >= other.x0
            && self.y0 - pad <= other.y1
            && self.y1 + pad >= other.y0
    }

    #[inline]
    pub fn horizontal_overlap(&self, other: &BBox) -> f32 {
        (self.x1.min(other.x1) - self.x0.max(other.x0)).max(0.0)
    }

    #[inline]
    pub fn vertical_overlap(&self, other: &BBox) -> f32 {
        (self.y1.min(other.y1) - self.y0.max(other.y0)).max(0.0)
    }

    #[inline]
    pub fn union(&self, other: &BBox) -> BBox {
        BBox {
            x0: self.x0.min(other.x0),
            y0: self.y0.min(other.y0),
            x1: self.x1.max(other.x1),
            y1: self.y1.max(other.y1),
        }
    }

    #[inline]
    pub fn expand(&self, pad_x: f32, pad_y: f32, max_w: f32, max_h: f32) -> BBox {
        BBox {
            x0: (self.x0 - pad_x).max(0.0),
            y0: (self.y0 - pad_y).max(0.0),
            x1: (self.x1 + pad_x).min(max_w),
            y1: (self.y1 + pad_y).min(max_h),
        }
    }
}

/// A positioned text span extracted from a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextSpan {
    pub text: String,
    pub bbox: BBox,
    pub font_size: f32,
    pub is_bold: bool,
    pub is_mono: bool,
    pub consumed: bool,
}

/// A horizontal line of assembled text spans on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextLine {
    pub text: String,
    pub bbox: BBox,
    pub font_size: f32,
    pub is_bold: bool,
    pub spans: Vec<TextSpan>,
}

/// An axis-aligned vector line segment (used for table border & grid reconstruction).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GridSegment {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub is_horizontal: bool,
}

/// Raw geometric and text primitives extracted from a single PDF page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawPageData {
    pub page_number: usize, // 1-based
    pub width: f32,
    pub height: f32,
    pub spans: Vec<TextSpan>,
    pub grid_segments: Vec<GridSegment>,
    pub graphic_bboxes: Vec<BBox>,
    pub image_bboxes: Vec<BBox>,
}

/// A bookmark entry from the PDF outline (Table of Contents).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TocEntry {
    pub id: String,
    pub level: usize,
    pub title: String,
    pub start_page: usize, // 1-based
    pub end_page: usize,   // 1-based inclusive
    pub slug: String,
}

/// A reconstructed table from vector grids or column alignment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedTable {
    pub id: String,
    pub caption: Option<String>,
    pub page_start: usize,
    pub page_end: usize,
    pub bbox: BBox,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
    pub section_file: String,
    pub line_number: usize,
}

/// A detected hardware figure (vector diagram or raster graphic) on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FigureRegion {
    pub id: String,
    pub caption: String,
    pub page_number: usize,
    pub bbox: BBox,
    pub image_rel_path: Option<String>,
    pub section_file: String,
    pub line_number: usize,
}

/// A structured peripheral register bitfield record for `index/registers.tsv`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterBitfield {
    pub peripheral: String,
    pub register: String,
    pub offset: String,
    pub reset_value: String,
    pub bits: String,
    pub field_name: String,
    pub access: String,
    pub field_reset: String,
    pub description: String,
    pub section_file: String,
    pub line_number: usize,
    pub page_number: usize,
}

/// A cross-reference graph entry for `index/xref.tsv` linking peripherals, registers,
/// bitfields, pins/table rows, figures, and chapters across the document.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct XrefEntry {
    pub symbol: String,
    pub kind: String,
    pub peripheral: String,
    pub summary: String,
    pub location: String,
    pub page_number: usize,
}

/// Ordered semantic elements assembled for a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageBlock {
    Heading {
        level: usize,
        text: String,
        page: usize,
        y0: f32,
    },
    RegisterHeading {
        section_num: String,
        peripheral: String,
        register: String,
        offset: String,
        reset: String,
        description: Option<String>,
        page: usize,
        y0: f32,
    },
    BitMapSummary {
        summary: String,
        page: usize,
        y0: f32,
    },
    Paragraph {
        text: String,
        page: usize,
        y0: f32,
    },
    Table {
        table: ExtractedTable,
        y0: f32,
    },
    Figure {
        figure: FigureRegion,
        y0: f32,
    },
}

impl PageBlock {
    pub fn y0(&self) -> f32 {
        match self {
            PageBlock::Heading { y0, .. }
            | PageBlock::RegisterHeading { y0, .. }
            | PageBlock::BitMapSummary { y0, .. }
            | PageBlock::Paragraph { y0, .. }
            | PageBlock::Table { y0, .. }
            | PageBlock::Figure { y0, .. } => *y0,
        }
    }

    pub fn page(&self) -> usize {
        match self {
            PageBlock::Heading { page, .. }
            | PageBlock::RegisterHeading { page, .. }
            | PageBlock::BitMapSummary { page, .. }
            | PageBlock::Paragraph { page, .. } => *page,
            PageBlock::Table { table, .. } => table.page_start,
            PageBlock::Figure { figure, .. } => figure.page_number,
        }
    }
}

/// Metadata extracted from the PDF header/info dictionary.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DocMetadata {
    pub file_name: String,
    pub doc_stem: String,
    #[serde(default)]
    pub source_hash: String,
    #[serde(default)]
    pub source_size_bytes: u64,
    pub title: Option<String>,
    pub author: Option<String>,
    pub subject: Option<String>,
    pub keywords: Option<String>,
    pub creator: Option<String>,
    pub producer: Option<String>,
    pub total_pages: usize,
    pub converted_pages: usize,
}

/// A sharded section file ready to be written to `sections/<slug>.md`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SectionFile {
    pub index: usize,
    pub section_id: String,
    pub title: String,
    pub level: usize,
    pub start_page: usize,
    pub end_page: usize,
    pub file_rel_path: String,
    pub markdown: String,
    pub line_count: usize,
    pub table_count: usize,
    pub register_count: usize,
    pub figure_count: usize,
}

/// Quality and sanity metrics written to `meta.json`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SanityReport {
    pub chipdoc_version: String,
    pub elapsed_ms: u128,
    pub total_pages: usize,
    pub converted_pages: usize,
    pub section_files_count: usize,
    pub tables_extracted: usize,
    pub multi_page_tables_stitched: usize,
    pub registers_indexed: usize,
    pub bitfields_indexed: usize,
    pub visual_bitboxes_deduplicated: usize,
    pub figures_detected: usize,
    pub figures_rendered: usize,
    pub figure_text_spans_suppressed: usize,
    pub header_footer_spans_stripped: usize,
    #[serde(default)]
    pub xrefs_indexed: usize,
    #[serde(default)]
    pub cache_hit: bool,
    pub replacement_char_count: usize,
    pub estimated_tokens: usize,
    pub sanity_score: u8, // 0..=100
    pub warnings: Vec<String>,
}

/// Configuration for `chipdoc convert`.
#[derive(Debug, Clone)]
pub struct ConvertConfig {
    pub input_pdf: PathBuf,
    pub output_root: PathBuf,
    pub page_filter: Option<Vec<usize>>, // Sorted 1-based page numbers
    pub single_file: bool,
    pub render_figures: bool,
    pub lazy_figures: bool,
    pub force: bool,
    pub update_claudeignore: bool,
    pub figure_dpi: f32,
    pub pdfium_lib_dir: Option<PathBuf>,
}

impl ConvertConfig {
    /// Creates a default conversion configuration for `input_pdf` into `output_root`.
    pub fn new(input_pdf: PathBuf, output_root: PathBuf) -> Self {
        Self {
            input_pdf,
            output_root,
            page_filter: None,
            single_file: false,
            render_figures: true,
            lazy_figures: false,
            force: false,
            update_claudeignore: true,
            figure_dpi: 200.0,
            pdfium_lib_dir: None,
        }
    }
}

/// Result returned from a full conversion run.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionBundle {
    pub bundle_dir: PathBuf,
    pub index_md_path: PathBuf,
    pub single_md_path: Option<PathBuf>,
    pub metadata: DocMetadata,
    pub sanity: SanityReport,
}
