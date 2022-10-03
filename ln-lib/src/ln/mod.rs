mod scrape;

use std::mem;
use std::ops::Index;
use std::slice::SliceIndex;
use std::vec::IntoIter;

use super::LightnovelChapter;
use crate::err::Error;
use scrape::{get_cha, get_cha_by_id};

#[allow(dead_code)]
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

	pub fn get_last_page(&self) -> Option<usize> {
		self.last_page
	}

	pub async fn scrape(&mut self) -> Result<bool, Error> {
		let (id, last_id, mut data) = match self.id {
			Some(id) => (id, None, get_cha_by_id(id, self.page).await?),
			None => get_cha(&self.url, None).await?,
		};

		self.id = Some(id);
		self.last_page = last_id;

		self.chapters = data
			.iter_mut()
			.enumerate()
			.map(|(i, (title, url))| LightnovelChapter::new(mem::take(title), mem::take(url), (self.page - 1) * 48 + i + 1))
			.collect();

		Ok(!self.chapters.is_empty())
	}

	pub async fn next_scrape(&mut self) -> Result<bool, Error> {
		self.next_page();
		self.scrape().await
	}

	pub async fn open_scrape(&mut self, page: usize) -> Result<bool, Error> {
		self.open_page(page);
		self.scrape().await
	}

	pub async fn prev_scrape(&mut self) -> Result<bool, Error> {
		self.prev_page();
		self.scrape().await
	}

	pub fn next_page(&mut self) {
		if self.page < self.last_page.unwrap_or(1) {
			self.page += 1;
		}
	}

	pub fn open_page(&mut self, page: usize) {
		if page <= self.last_page.unwrap_or(1) {
			self.page = page;
		}
	}

	pub fn prev_page(&mut self) {
		if self.page > 1 {
			self.page -= 1;
		}
	}
}

impl<Idx> Index<Idx> for Lightnovel
where
	Idx: SliceIndex<[LightnovelChapter]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.chapters[index]
	}
}

impl IntoIterator for Lightnovel {
	type Item = LightnovelChapter;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.chapters.into_iter()
	}
}

impl IntoIterator for &Lightnovel {
	type Item = LightnovelChapter;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.chapters.clone().into_iter()
	}
}