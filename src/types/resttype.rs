use napi_derive::napi;
use serde::Deserialize;
#[derive(Deserialize)]
#[napi(object)]
pub struct Track {
  pub encoded: String,
  pub info: TrackInfo,
  #[napi(js_name = "pluginInfo")]
  pub plugin_info: serde_json::Value,
  #[napi(js_name = "userData")]
  pub user_data: serde_json::Value,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct TrackInfo {
  pub identifier: String,
  #[napi(js_name = "isSeekable")]
  pub is_seekable: bool,
  pub author: String,
  pub length: i64,
  #[napi(js_name = "isStream")]
  pub is_stream: bool,
  pub position: i64,
  pub title: String,
  pub uri: Option<String>,
  #[napi(js_name = "artworkUrl")]
  pub artwork_url: Option<String>,
  pub isrc: Option<String>,
  #[napi(js_name = "sourceName")]
  pub source_name: String,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct Exception {
  pub message: String,
  pub severity: String,
  pub cause: String,
}

#[napi(object)]
pub struct Playlist {
  pub name: String,
  #[napi(js_name = "selectedTrack")]
  pub selected_track: i32,
  pub tracks: Vec<Track>,
  #[napi(js_name = "pluginInfo")]
  pub plugin_info: serde_json::Value,
}

pub enum LoadType {
  Track(Track),
  Playlist(Playlist),
  Search(Vec<Track>),
  Empty,
  Error(Exception),
}
