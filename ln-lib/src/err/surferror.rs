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
		fmt.write_str("Surf error: There is an error acourred while using the lib surf")
	}
}

impl Context for SurfError {}
