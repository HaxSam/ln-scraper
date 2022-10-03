mod scrape;

use std::ops::Index;
use std::slice::SliceIndex;
use std::vec::IntoIter;

use crate::err::Error;

#[derive(Default, Debug, Clone)]
pub struct LightnovelChapter {
	title: String,
	url: String,
	chapter_number: usize,
	paragraph: Vec<String>,
}

impl LightnovelChapter {
	pub fn new(title: String, url: String, chapter_number: usize) -> Self {
		Self {
			title,
			url,
			chapter_number,
			..Default::default()
		}
	}

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_url(&self) -> &String {
		&self.url
	}

	pub fn get_chapter_number(&self) -> usize {
		self.chapter_number
	}

	pub fn len(&self) -> usize {
		self.paragraph.len()
	}

	pub async fn scrape(&mut self) -> Result<bool, Error> {
		self.paragraph = scrape::get_paragraph(&self.url).await?;
		Ok(!self.paragraph.is_empty())
	}
}

impl<Idx> Index<Idx> for LightnovelChapter
where
	Idx: SliceIndex<[String]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.paragraph[index]
	}
}

impl IntoIterator for LightnovelChapter {
	type Item = String;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.paragraph.into_iter()
	}
}

impl IntoIterator for &LightnovelChapter {
	type Item = String;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.paragraph.clone().into_iter()
	}
}
