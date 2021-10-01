use serde::Serialize;

use super::Vehicle;
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PointTO {
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

impl PointTO {
	#[allow(clippy::too_many_arguments)]
	pub fn new(
		id: i32,
		x: i32,
		y: i32,
		z: i32,
		length: i32,
		width: i32,
		height: i32,
		weight_class: i32,
		order_class: i32,
	) -> Self {
		Self {
			id,
			x1: x,
			x2: x,
			x3: x,
			x4: x,
			x5: x + length,
			x6: x + length,
			x7: x + length,
			x8: x + length,
			y1: y,
			y2: y,
			y3: y,
			y4: y,
			y5: y + width,
			y6: y + width,
			y7: y + width,
			y8: y + width,
			z1: z,
			z2: z,
			z3: z,
			z4: z,
			z5: z + height,
			z6: z + height,
			z7: z + height,
			z8: z + height,
			weight_class,
			order_class,
		}
	}
}
