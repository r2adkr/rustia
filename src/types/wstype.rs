use crate::types::resttype::{Exception, Track};
use napi_derive::napi;
use serde::Deserialize;
#[derive(Deserialize)]
#[napi(object)]
pub struct ReadyEvent {
  pub op: String,
  pub resumed: bool,
  #[napi(js_name = "sessionId")]
  pub session_id: String,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct PlayerUpdate {
  pub op: String,
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub state: PlayerState,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct PlayerState {
  pub time: i64,
  pub position: i64,
  pub connected: bool,
  pub ping: i16,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct MemoryStats {
  pub free: i64,
  pub used: i64,
  pub allocated: i64,
  pub reservable: i64,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct CpuStats {
  pub cores: i32,
  #[napi(js_name = "systemLoad")]
  pub system_load: f64,
  #[napi(js_name = "processLoad")]
  pub process_load: f64,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct FrameStats {
  pub sent: i32,
  pub nulled: i32,
  pub deficit: i32,
  pub expected: i32,
}

#[derive(Deserialize)]
#[napi(object)]
pub struct StatsEvent {
  pub players: i32,
  #[napi(js_name = "playingPlayers")]
  pub playing_players: i32,
  pub uptime: i64,
  pub memory: MemoryStats,
  pub cpu: CpuStats,
  #[napi(js_name = "frameStats")]
  pub frame_stats: Option<FrameStats>,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct TrackStartEvent {
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub track: Track,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct TrackEndEvent {
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub track: Track,
  pub reason: String,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct TrackExceptionEvent {
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub track: Track,
  pub exception: Exception,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct TrackStuckEvent {
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub track: Track,
  #[napi(js_name = "thresholdMs")]
  pub threshold_ms: i64,
}
#[derive(Deserialize)]
#[napi(object)]
pub struct WebSocketClosedEvent {
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub code: i32,
  pub reason: String,
  #[napi(js_name = "byRemote")]
  pub by_remote: bool,
}

#[napi(object)]
pub struct PlayerUpdateEvent {
  #[napi(js_name = "guildId")]
  pub guild_id: String,
  pub state: PlayerState,
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "camelCase")]
pub enum NodeMessage {
  Ready(ReadyEvent),
  Stats(StatsEvent),
  PlayerUpdate(serde_json::Value),
  #[serde(rename = "event")]
  Event(EventPayload),
}

#[derive(Deserialize)]
#[serde(tag = "type", rename_all = "camelCase")]
pub enum EventPayload {
  TrackStartEvent(TrackStartEvent),
  TrackEndEvent(TrackEndEvent),
  TrackExceptionEvent(TrackExceptionEvent),
  TrackStuckEvent(TrackStuckEvent),
  WebSocketClosedEvent(WebSocketClosedEvent),
}
