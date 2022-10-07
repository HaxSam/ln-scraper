mod scrape;

use std::mem;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

use error_stack::Result;

use super::LightnovelChapter;
use crate::err::LightnovelError;
use scrape::{get_cha, get_cha_by_id};

#[derive(Debug, Clone)]
pub struct Lightnovel {
	id: Option<usize>,
	title: String,
	url: String,
	page: usize,
	last_page: Option<usize>,
	chapters: Vec<LightnovelChapter>,
}

impl Default for Lightnovel {
	fn default() -> Self {
		Self {
			id: None,
			title: String::new(),
			url: String::new(),
			page: 1,
			last_page: None,
			chapters: Vec::with_capacity(48),
		}
	}
}

impl Lightnovel {
	pub fn new(title: String, url: String) -> Self {
		Self {
			title,
			url,
			..Default::default()
		}
	}

	pub fn get_id(&self) -> Option<usize> {
		self.id
	}

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_url(&self) -> &String {
		&self.url
	}

	pub fn get_page(&self) -> usize {
		self.page
	}

	pub fn get_last_page(&self) -> Option<usize> {
		self.last_page
	}

	pub async fn scrape(&mut self) -> Result<(), LightnovelError> {
		let (id, last_page, mut data) = match self.id {
			Some(id) => (id, self.last_page, get_cha_by_id(id, self.page).await?),
			None => get_cha(&self.url, None).await?,
		};

		self.id = Some(id);
		self.last_page = last_page;

		self.chapters = data
			.iter_mut()
			.enumerate()
			.map(|(i, (title, url))| LightnovelChapter::new(mem::take(title), mem::take(url), (self.page - 1) * 48 + i + 1))
			.collect();

		Ok(())
	}

	pub async fn next_scrape(&mut self) -> Result<bool, LightnovelError> {
		if let None = self.next_page() {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(self.page != self.last_page.unwrap_or(1))
	}

	pub async fn open_scrape(&mut self, page: usize) -> Result<bool, LightnovelError> {
		if let None = self.open_page(page) {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(!self.chapters.is_empty())
	}

	pub async fn prev_scrape(&mut self) -> Result<bool, LightnovelError> {
		if let None = self.prev_page() {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(self.page != 1)
	}

	pub fn next_page(&mut self) -> Option<usize> {
		self.open_page(self.page + 1)
	}

	pub fn open_page(&mut self, page: usize) -> Option<usize> {
		if page <= self.last_page.unwrap_or(1) && page >= 1 {
			self.page = page;
			Some(page)
		} else {
			None
		}
	}

	pub fn prev_page(&mut self) -> Option<usize> {
		self.open_page(self.page - 1)
	}
}

impl IntoIterator for Lightnovel {
	type Item = LightnovelChapter;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.chapters.into_iter()
	}
}

impl Deref for Lightnovel {
	type Target = Vec<LightnovelChapter>;

	fn deref(&self) -> &Self::Target {
		&self.chapters
	}
}

impl DerefMut for Lightnovel {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.chapters
	}
}
