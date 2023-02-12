use super::SurfError;
use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum ChapterError {
	ScraperError(SurfError),
}

impl fmt::Display for ChapterError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err_text = match self {
			Self::ScraperError(_) => format!("Chapter error: There is an error acourred while trying to scrape the content from a chapter"),
		};

		fmt.write_str(&err_text)
	}
}

impl Context for ChapterError {}

impl From<SurfError> for ChapterError {
	fn from(err: SurfError) -> Self {
		ChapterError::ScraperError(err)
	}
}
