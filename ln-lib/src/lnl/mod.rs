mod scrape;

use std::mem;
use std::ops::Index;
use std::slice::SliceIndex;
use std::vec::IntoIter;

use surf::{middleware::Redirect, Client, Config, Url};

use super::Lightnovel;
use super::LightnovelCategory;
use crate::cnf::{CLIENT, LIGHTNOVEL_SITE};
use crate::err::Error;
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
	pub fn new(category: LightnovelCategory) -> Result<Self, Error> {
		if let None = CLIENT.get() {
			let url = Url::parse(LIGHTNOVEL_SITE)?;
			let config = Config::new().set_base_url(url);

			let tryclient: Result<Client, _> = config.try_into();

			match tryclient {
				Ok(client) => CLIENT.set(client.with(Redirect::new(3))).unwrap(),
				Err(_) => panic!("There is a deep problem while converting the config into the surf client pls create a issue"),
			};
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

	pub async fn scrape(&mut self) -> Result<(), Error> {
		use LightnovelCategory::*;

		let url = match &self.category {
			Latest => format!("/latest/page/{}", self.page),
			Completed => format!("/completed/page/{}", self.page),
			Genre(g) => format!("/{}/page/{}", g, self.page),
			Title(t) => format!("/page/{}?s={}", self.page, t),
		};

		let (mut data, last_page) = get_ln(url).await?;

		self.last_page = last_page;
		self.list = data
			.iter_mut()
			.map(|(title, url)| Lightnovel::new(mem::take(title), mem::take(url)))
			.collect();

		Ok(())
	}

	pub async fn next_scrape(&mut self) -> Result<bool, Error> {
		if let None = self.next_page() {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(self.page != self.last_page.unwrap_or(1))
	}

	pub async fn open_scrape(&mut self, page: usize) -> Result<bool, Error> {
		if let None = self.open_page(page) {
			return Ok(false);
		}
		self.scrape().await?;
		Ok(!self.list.is_empty())
	}

	pub async fn prev_scrape(&mut self) -> Result<bool, Error> {
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

impl<Idx> Index<Idx> for LightnovelList
where
	Idx: SliceIndex<[Lightnovel]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.list[index]
	}
}

impl IntoIterator for LightnovelList {
	type Item = Lightnovel;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.list.into_iter()
	}
}

impl IntoIterator for &LightnovelList {
	type Item = Lightnovel;
	type IntoIter = IntoIter<Self::Item>;

	fn into_iter(self) -> Self::IntoIter {
		self.list.clone().into_iter()
	}
}
