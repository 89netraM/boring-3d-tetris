use super::api::{DimPackage, PointTO, Vehicle};

// Algorithm "borrowed" from the C# starter kit.

pub struct GreedySolver<'a> {
	normal_packages: Vec<&'a DimPackage>,
	heavy_packages: Vec<&'a DimPackage>,
	placed_packages: Vec<i32>,
	solution: Vec<PointTO>,
	truck_x: i32,
	truck_y: i32,
	truck_z: i32,
	xp: i32,
	yp: i32,
	zp: i32,
	last_known_max_width: i32,
	last_known_max_length: i32,
}

impl<'a> GreedySolver<'a> {
	pub fn solve(packets: &'a [DimPackage], truck: &Vehicle) -> Vec<PointTO> {
		let mut gs = Self::new(packets, truck);

		gs.sort_by_max_area();

		while gs.placed_packages.len() < gs.normal_packages.len() + gs.heavy_packages.len() {
			let package = match (gs.get_next_normal_package(), gs.get_next_heavy_package()) {
				(Some(next_normal), Some(next_heavy)) => {
					if (!gs.does_package_fit_z(next_normal) && !gs.does_package_fit_z(next_heavy))
						|| gs.zp
							<= gs
								.heavy_packages
								.iter()
								.map(|p| p.height)
								.max()
								.unwrap_or(0)
					{
						next_heavy
					} else {
						next_normal
					}
				}
				(Some(next_normal), None) => next_normal,
				(None, Some(next_heavy)) => next_heavy,
				_ => panic!("No packages left to place"),
			}
			.clone();

			if gs.does_package_fit_z(&package) {
				gs.add_package(&package);
				gs.zp += package.height;
			} else if gs.does_package_fit_y(&package) {
				gs.yp += gs.last_known_max_width;
				gs.zp = 0;
				gs.add_package(&package);
				gs.zp = package.height;
				gs.last_known_max_width = 0;
			} else if gs.does_package_fit_x(&package) {
				gs.xp += gs.last_known_max_length;
				gs.yp = 0;
				gs.zp = 0;
				gs.add_package(&package);
				gs.zp = package.height;
				gs.last_known_max_length = 0;
			} else {
				panic!("No space for package");
			}

			gs.set_max_x(&package);
			gs.set_max_y(&package);
		}

		gs.solution
	}

	fn new(packets: &'a [DimPackage], truck: &Vehicle) -> GreedySolver<'a> {
		GreedySolver {
			normal_packages: packets.iter().filter(|p| p.weight_class != 2).collect(),
			heavy_packages: packets.iter().filter(|p| p.weight_class == 2).collect(),
			placed_packages: Vec::new(),
			solution: Vec::new(),
			truck_x: truck.length,
			truck_y: truck.width,
			truck_z: truck.height,
			xp: 0,
			yp: 0,
			zp: 0,
			last_known_max_width: 0,
			last_known_max_length: 0,
		}
	}

	fn set_max_x(&mut self, p: &DimPackage) {
		if p.length > self.last_known_max_length {
			self.last_known_max_length = p.length;
		}
	}

	fn set_max_y(&mut self, p: &DimPackage) {
		if p.width > self.last_known_max_width {
			self.last_known_max_width = p.width;
		}
	}

	fn does_package_fit_x(&self, p: &DimPackage) -> bool {
		self.xp + self.last_known_max_length + p.length < self.truck_x
	}

	fn does_package_fit_y(&self, p: &DimPackage) -> bool {
		self.yp + self.last_known_max_width + p.width < self.truck_y
			&& self.xp + p.length < self.truck_x
	}

	fn does_package_fit_z(&self, p: &DimPackage) -> bool {
		self.xp + p.length < self.truck_x
			&& self.yp + p.width < self.truck_y
			&& self.zp + p.height < self.truck_z
	}

	fn sort_by_max_area(&mut self) {
		self.normal_packages.sort_by_key(|p| -p.width * p.length);
		self.heavy_packages.sort_by_key(|p| -p.width * p.length);
	}

	fn get_next_normal_package(&self) -> Option<&'a DimPackage> {
		self.normal_packages
			.iter()
			.find(|p| !self.placed_packages.contains(&p.id))
			.copied()
	}

	fn get_next_heavy_package(&self) -> Option<&'a DimPackage> {
		self.heavy_packages
			.iter()
			.find(|p| !self.placed_packages.contains(&p.id))
			.copied()
	}

	fn add_package(&mut self, p: &DimPackage) {
		self.solution.push(p.place(self.xp, self.yp, self.zp));
		self.placed_packages.push(p.id);
	}
}
