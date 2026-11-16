use crate::qobuz_models::track::Track;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicListInfo {
    pub title: String,
    #[serde(default)]
    pub baseline: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicList {
    pub title: String,
    #[serde(default)]
    pub baseline: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub kind: String,
    #[serde(default)]
    pub track_count: Option<u32>,
    #[serde(default)]
    pub generated_at: Option<i64>,
    #[serde(default)]
    pub expires_on: Option<i64>,
    #[serde(default)]
    pub tracks: DynamicListTracks,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DynamicListTracks {
    #[serde(default)]
    pub items: Vec<Track>,
}
