use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DimPackage {
	pub id: i32,
	pub width: i32,
	pub length: i32,
	pub height: i32,
	pub weight_class: i32,
	pub order_class: i32,
}
