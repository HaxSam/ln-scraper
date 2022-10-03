use surf::Error as SurfError;
use thiserror::Error;
use url::ParseError;

#[derive(Error, Debug)]
pub enum Error {
	#[error("There was a problem with parsing the url: {0}")]
	ParseError(#[from] ParseError),

	#[error("There was a problem with surf: {0}")]
	SurfError(String),

	#[error("Coudnt get the ID from chapter")]
	GetIdError,
}

impl From<SurfError> for Error {
	fn from(err: SurfError) -> Error {
		Error::SurfError(err.to_string())
	}
}
