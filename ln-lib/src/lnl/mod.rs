mod scrape;

use std::mem;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;

use error_stack::{IntoReport, Result, ResultExt};
use surf::{middleware::Redirect, Client, Config, Url};

use super::Lightnovel;
use super::LightnovelCategory;
use crate::cfg::{CLIENT, LIGHTNOVEL_SITE};
use crate::err::{ListError, SurfError};
use scrape::get_ln;

#[derive(Debug)]
pub struct LightnovelList {
	category: LightnovelCategory,
	page: usize,
	last_page: Option<usize>,
	list: Vec<Lightnovel>,
}

impl Default for LightnovelList {
	fn default() -> Self {
		Self {
			category: LightnovelCategory::default(),
			page: 1,
			last_page: None,
			list: Vec::with_capacity(24),
		}
	}
}

impl LightnovelList {
	pub fn new(category: LightnovelCategory) -> Result<Self, ListError> {
		if let None = CLIENT.get() {
			let url = Url::parse(LIGHTNOVEL_SITE)
				.into_report()
				.change_context(SurfError::UriParserError.into())?;
			let config = Config::new().set_base_url(url);

			let client: Client = config.try_into().into_report().change_context(SurfError::ClientCreationError.into())?;

			CLIENT.set(client.with(Redirect::new(3))).unwrap();
		}

		Ok(Self {
			category,
			..Default::default()
		})
	}

	pub fn category(&self) -> &LightnovelCategory {
		&self.category
	}

	pub fn page(&self) -> usize {
		self.page
	}

	pub fn last_page(&self) -> Option<usize> {
		self.last_page
	}

	pub async fn scrape(&mut self) -> Result<(), ListError> {
		use LightnovelCategory::*;

		let url = match &self.category {
			Latest => format!("/latest/page/{}", self.page),
			Completed => format!("/completed/page/{}", self.page),
			Genre(g) => format!("/{}/page/{}", g, self.page),
			Title(t) => format!("/page/{}?s={}", self.page, t),
		};

		let (mut data, last_page) = get_ln(&url).await?;

		self.last_page = last_page;
		self.list = data
			.iter_mut()
			.map(|(title, url)| Lightnovel::new(mem::take(title), mem::take(url)))
			.collect();

		Ok(())
	}

	pub async fn next_scrape(&mut self) -> Result<bool, ListError> {
		if let None = self.next_page() {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(self.page != self.last_page.unwrap_or(1))
	}

	pub async fn open_scrape(&mut self, page: usize) -> Result<bool, ListError> {
		if let None = self.open_page(page) {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(!self.list.is_empty())
	}

	pub async fn prev_scrape(&mut self) -> Result<bool, ListError> {
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

impl IntoIterator for LightnovelList {
	type Item = Lightnovel;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.list.into_iter()
	}
}

impl Deref for LightnovelList {
	type Target = Vec<Lightnovel>;

	fn deref(&self) -> &Self::Target {
		&self.list
	}
}

impl DerefMut for LightnovelList {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.list
	}
}
