use serde::Deserialize;

use super::{GameStats, PointPackage};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SavedGameResponse {
	pub game_id: String,
	pub team_name: Option<String>,
	pub score: i32,
	pub map: Option<String>,
	pub completed_at_time: String,
	pub game_stats: GameStats,
	pub submission: Option<Vec<PointPackage>>,
}
