use std::path::PathBuf;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ChipdocError {
    #[error("PDF file not found: {0}")]
    PdfNotFound(PathBuf),

    #[error("No converted chipdoc bundle found at '{0}'. Run `chipdoc ensure <pdf>` or `chipdoc convert <pdf>` first.")]
    BundleNotFound(PathBuf),

    #[error("Failed to bind PDFium library: {0}. Run `pwsh scripts/fetch_pdfium.ps1` to stage bin/pdfium.dll.")]
    PdfiumBind(String),

    #[error("PDFium error while processing '{path}': {message}")]
    Pdfium { path: String, message: String },

    #[error("Invalid page range expression '{0}'")]
    InvalidPageRange(String),

    #[error("I/O error at '{path}': {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Image encoding error for '{path}': {message}")]
    Image { path: PathBuf, message: String },
}

pub type Result<T> = std::result::Result<T, ChipdocError>;
