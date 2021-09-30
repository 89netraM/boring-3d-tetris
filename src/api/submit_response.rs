use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitResponse {
	pub game_id: String,
	pub score: i32,
	pub valid: bool,
	pub link: Option<String>,
}
