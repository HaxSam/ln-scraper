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
		let err_text = match self {
			Self::GetIDError => format!("Lightnovel error: There is an error acourred while trying to get the Lightnovel ID"),
			Self::ScraperError(_) => format!("Lightnovel error: There is an error acourred while trying to scrape"),
		};

		fmt.write_str(&err_text)
	}
}

impl Context for LightnovelError {}

impl From<SurfError> for LightnovelError {
	fn from(err: SurfError) -> Self {
		LightnovelError::ScraperError(err)
	}
}
