use error_stack::Context;
use std::fmt;

#[derive(Debug)]
pub enum SurfError {
	RequestError(String),
	BodyParseError(String),
	ClientCreationError,
	UriParserError,
}

impl fmt::Display for SurfError {
	fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
		let err_text = match self {
			Self::RequestError(_) => format!("Surf error: There acourred a request error"),
			Self::BodyParseError(_) => format!("Surf error: There acourred a error while parsing the body"),
			Self::ClientCreationError => format!("Surf error: There acourred a error while creating a client"),
			Self::UriParserError => format!("Surf error: There acourred a error while parsing the URI"),
		};

		fmt.write_str(&err_text)
	}
}

impl Context for SurfError {}
