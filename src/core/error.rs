use thiserror::Error;

#[derive(Debug, Error)]
pub enum SDFError {
    #[error("Invalid channel configuration: {0}")]
    InvalidChannelConfig(String),
    
    #[error("Channel dimension mismatch: expected {expected}x{expected_h}, got {actual_w}x{actual_h}")]
    DimensionMismatch {
        expected: u32,
        expected_h: u32,
        actual_w: u32,
        actual_h: u32,
    },
    
    #[error("Unsupported image format: {format}")]
    UnsupportedFormat { format: String },
    
    #[error("Processing failed: {reason}")]
    ProcessingFailed { reason: String },
    
    #[error("Memory allocation failed: requested {requested} bytes")]
    OutOfMemory { requested: usize },
    
    #[error("Channel validation failed: {details}")]
    ValidationError { details: String },
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Image error: {0}")]
    ImageError(#[from] image::ImageError),
}

impl SDFError {
    pub fn recovery_suggestion(&self) -> Option<String> {
        match self {
            SDFError::DimensionMismatch { .. } => {
                Some("Try resizing all input channels to the same dimensions".to_string())
            },
            SDFError::OutOfMemory { .. } => {
                Some("Try reducing the image size or using streaming processing".to_string())
            },
            SDFError::UnsupportedFormat { format } => {
                Some(format!("Convert {} to PNG, JPEG, or TGA format", format))
            },
            _ => None,
        }
    }
}