use serde::{Deserialize, Serialize};
use sqlx::Type;
use std::str::FromStr;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Type)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "conversion_type", rename_all = "snake_case")]
pub enum ConversionType {
    ImageToImage,
    DocumentToPdf,
    VideoToVideo,
    AudioToAudio,
}

impl std::fmt::Display for ConversionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ImageToImage => write!(f, "image_to_image"),
            Self::DocumentToPdf => write!(f, "document_to_pdf"),
            Self::VideoToVideo => write!(f, "video_to_video"),
            Self::AudioToAudio => write!(f, "audio_to_audio"),
        }
    }
}

impl FromStr for ConversionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "image_to_image" => Ok(Self::ImageToImage),
            "document_to_pdf" => Ok(Self::DocumentToPdf),
            "video_to_video" => Ok(Self::VideoToVideo),
            "audio_to_audio" => Ok(Self::AudioToAudio),
            _ => Err(format!("Unknown ConversionType variant: {}", s)),
        }
    }
}

impl Default for ConversionType {
    fn default() -> Self {
        Self::ImageToImage
    }
}
