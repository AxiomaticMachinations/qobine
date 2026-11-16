use crate::qobuz_models::{
    Image, album_suggestion::Label, artist::Artist, featured::Award, track::Tracks,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Album {
    pub artist: Artist,
    pub description: Option<String>,
    pub duration: Option<i64>,
    pub hires_streamable: bool,
    #[serde(default)]
    pub maximum_bit_depth: Option<u32>,
    #[serde(default)]
    pub maximum_sampling_rate: Option<f32>,
    pub id: String,
    pub image: Image,
    #[serde(default)]
    pub parental_warning: bool,
    pub release_date_original: String,
    pub streamable: bool,
    pub title: String,
    pub tracks: Option<Tracks>,
    #[serde(default)]
    pub tracks_count: i64,
    #[serde(default)]
    pub awards: Vec<Award>,
    pub label: Option<Label>,
    #[serde(default)]
    pub genre: Option<AlbumGenre>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AlbumGenre {
    pub id: u32,
    #[serde(default)]
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AlbumSearchResults {
    pub query: String,
    pub albums: Albums,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Albums {
    pub limit: i64,
    pub offset: i64,
    pub total: i64,
    pub items: Vec<Album>,
}
