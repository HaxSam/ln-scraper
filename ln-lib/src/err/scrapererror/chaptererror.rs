use super::SurfError;
use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum ChapterError {
	ScraperError(SurfError),
}

impl fmt::Display for ChapterError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt.write_str("Chapter error: There is an error acourred while trying to scrape the content from a chapter")
	}
}

impl Context for ChapterError {}

impl From<SurfError> for ChapterError {
	fn from(err: SurfError) -> Self {
		ChapterError::ScraperError(err)
	}
}
