use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PointPackage {
	pub id: i32,
	pub x1: i32,
	pub x2: i32,
	pub x3: i32,
	pub x4: i32,
	pub x5: i32,
	pub x6: i32,
	pub x7: i32,
	pub x8: i32,
	pub y1: i32,
	pub y2: i32,
	pub y3: i32,
	pub y4: i32,
	pub y5: i32,
	pub y6: i32,
	pub y7: i32,
	pub y8: i32,
	pub z1: i32,
	pub z2: i32,
	pub z3: i32,
	pub z4: i32,
	pub z5: i32,
	pub z6: i32,
	pub z7: i32,
	pub z8: i32,
	pub weight_class: i32,
	pub order_class: i32,
}
