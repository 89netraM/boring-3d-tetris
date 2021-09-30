use serde::Deserialize;

use super::{DimPackage, Vehicle};

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Map {
	pub map_name: Option<String>,
	pub vehicle: Vehicle,
	pub dimensions: Option<Vec<DimPackage>>,
}
