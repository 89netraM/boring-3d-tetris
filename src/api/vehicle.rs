use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Vehicle {
	pub length: i32,
	pub width: i32,
	pub height: i32,
}
