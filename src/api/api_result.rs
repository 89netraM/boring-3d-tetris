use anyhow::{Error, Result};
use serde::Deserialize;

use super::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)]
pub enum ApiResult<T> {
	Ok(T),
	Err(ApiError),
}

impl<T> From<ApiResult<T>> for Result<T> {
	fn from(api_result: ApiResult<T>) -> Self {
		match api_result {
			ApiResult::Ok(t) => Ok(t),
			ApiResult::Err(e) => Err(Error::new(e)),
		}
	}
}
