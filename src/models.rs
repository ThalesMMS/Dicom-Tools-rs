use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicMetadata {
    pub patient_name: Option<String>,
    pub patient_id: Option<String>,
    pub study_date: Option<String>,
    pub modality: Option<String>,
    pub sop_class_uid: Option<String>,
    pub has_pixel_data: bool,
    pub transfer_syntax: Option<String>,
    pub rows: Option<u32>,
    pub columns: Option<u32>,
    pub number_of_frames: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedMetadata {
    pub patient: BTreeMap<String, String>,
    pub study: BTreeMap<String, String>,
    pub image: BTreeMap<String, String>,
    pub misc: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationSummary {
    pub valid: bool,
    pub missing_tags: Vec<String>,
    pub has_pixel_data: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PixelStatistics {
    pub min: f32,
    pub max: f32,
    pub mean: f32,
    pub median: Option<f32>,
    pub std_dev: f32,
    pub total_pixels: usize,
    pub shape: Vec<usize>,
}
