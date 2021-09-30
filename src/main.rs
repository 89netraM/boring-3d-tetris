mod api;
mod greedy_solver;

use anyhow::{anyhow, Result};
use greedy_solver::GreedySolver;

#[tokio::main]
async fn main() -> Result<()> {
	let key = std::env::args()
		.nth(1)
		.ok_or_else(|| anyhow!("Give API key as argument!"))?;

	let map = api::new_game(&key, &"training1").await?;
	println!("{:#?}", map);

	let point_tos = GreedySolver::solve(&map.dimensions.unwrap(), &map.vehicle);
	println!("{:#?}", point_tos);

	let res = api::submit_game(&key, &map.map_name.unwrap(), &point_tos).await?;
	println!("{:#?}", res);

	println!("{:#?}", api::fetch_game(&key, &res.game_id).await?);

	Ok(())
}
