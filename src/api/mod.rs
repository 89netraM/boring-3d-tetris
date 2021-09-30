mod api_error;
mod api_result;
mod dim_package;
mod game_stats;
mod map;
mod point_package;
mod point_to;
mod saved_game_response;
mod submit_response;
mod vehicle;

use anyhow::Result;
pub use api_error::ApiError;
use api_result::ApiResult;
pub use dim_package::DimPackage;
pub use game_stats::GameStats;
pub use map::Map;
pub use point_package::PointPackage;
pub use point_to::PointTO;
pub use saved_game_response::SavedGameResponse;
pub use submit_response::SubmitResponse;
pub use vehicle::Vehicle;

const BASE_URL: &str = "https://game.considition.com/api/games/";

pub async fn new_game(key: &str, version: &str) -> Result<Map> {
	reqwest::Client::new()
		.get(&format!("{}new?MapName={}", BASE_URL, version))
		.header("x-api-key", key)
		.send()
		.await?
		.json::<ApiResult<Map>>()
		.await?
		.into()
}

pub async fn submit_game(key: &str, version: &str, points: &[PointTO]) -> Result<SubmitResponse> {
	reqwest::Client::new()
		.post(&format!("{}submit?MapName={}", BASE_URL, version))
		.header("x-api-key", key)
		.json(points)
		.send()
		.await?
		.json::<ApiResult<SubmitResponse>>()
		.await?
		.into()
}

pub async fn fetch_game(key: &str, id: &str) -> Result<SavedGameResponse> {
	reqwest::Client::new()
		.get(&format!("{}fetch?GameId={}", BASE_URL, id))
		.header("x-api-key", key)
		.send()
		.await?
		.json::<ApiResult<SavedGameResponse>>()
		.await?
		.into()
}
