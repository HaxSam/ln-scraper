use super::SurfError;
use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum ListError {
	ScraperError(SurfError),
}

impl fmt::Display for ListError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str("List error: There is an error acourred while trying to scrape the lightnovels")
	}
}

impl Context for ListError {}

impl From<SurfError> for ListError {
	fn from(err: SurfError) -> Self {
		ListError::ScraperError(err)
	}
}
