use std::fmt::{Display, Formatter, Result};

#[derive(Default, Debug)]
pub enum LightnovelCategory {
	#[default]
	Latest,
	Completed,
	Genre(String),
	Title(String),
}

impl Display for LightnovelCategory {
	fn fmt(&self, f: &mut Formatter) -> Result {
		match self {
			LightnovelCategory::Latest => write!(f, "Latest"),
			LightnovelCategory::Completed => write!(f, "Completed"),
			LightnovelCategory::Genre(genre) => write!(f, "Genre: {}", genre),
			LightnovelCategory::Title(title) => write!(f, "Title: {}", title),
		}
	}
}
