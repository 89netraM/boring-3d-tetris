use serde::Deserialize;

use super::PointTO;

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

impl DimPackage {
	/// "Places" this package in the given location and returns a `PointTO`
	/// representing the placement.
	pub fn place(&self, x: i32, y: i32, z: i32) -> PointTO {
		PointTO::new(
			self.id,
			x,
			y,
			z,
			self.length,
			self.width,
			self.height,
			self.weight_class,
			self.order_class,
		)
	}
}
