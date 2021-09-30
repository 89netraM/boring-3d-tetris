use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameStats {
	pub score: i32,
	pub packed_length: f64,
	pub crushed_packages: i32,
	pub packing_efficiency_multiplier: f64,
	pub packing_efficiency: f64,
	pub order_score: f64,
	pub weight_score: f64,
	pub max_length_score: f64,
	pub max_order_score: f64,
	pub fragile_packages_count: i32,
}
