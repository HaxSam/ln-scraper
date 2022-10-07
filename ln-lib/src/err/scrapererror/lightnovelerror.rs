use super::SurfError;
use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum LightnovelError {
	ScraperError(SurfError),
	GetIDError,
}

impl fmt::Display for LightnovelError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str("Lightnovel error: There is an error acourred while trying to scrape the chapters from a lightnovel")
	}
}

impl Context for LightnovelError {}

impl From<SurfError> for LightnovelError {
	fn from(err: SurfError) -> Self {
		LightnovelError::ScraperError(err)
	}
}
