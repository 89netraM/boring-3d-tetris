use std::{
	error::Error,
	fmt::{Display, Formatter},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
	pub title: Option<String>,
	pub status: Option<i32>,
	pub detail: Option<String>,
}

impl Display for ApiError {
	fn fmt(&self, f: &mut Formatter) -> std::result::Result<(), std::fmt::Error> {
		write!(
			f,
			"{} ({})\n\t{}",
			self.title.as_ref().unwrap_or(&"".to_string()),
			self.status.as_ref().unwrap_or(&0),
			self.detail.as_ref().unwrap_or(&"".to_string())
		)
	}
}

impl Error for ApiError {
}
