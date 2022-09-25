mod scrape;

use std::{clone::Clone, default::Default, error::Error, fmt, iter::Iterator, mem};

use surf::{Client, Config, Url};

#[derive(Default)]
pub struct LightnovelList {
	client: Client,
	category: LightnovelCategory,
	page: i32,
	list: Vec<Lightnovel>,
}

#[derive(Default, Clone)]
pub struct Lightnovel {
	id: Option<i32>,
	title: String,
	url: String,
	page: i32,
	last_page: Option<i32>,
	chapters: Vec<LightnovelChapter>,
}

#[derive(Default, Clone)]
pub struct LightnovelChapter {
	title: String,
	url: String,
	chapter_number: i32,
	paragraph: Vec<String>,
}

#[derive(Default)]
pub enum LightnovelCategory {
	#[default]
	Latest,
	Completed,
	Genre(String),
	Title(String),
}

impl fmt::Display for LightnovelCategory {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			LightnovelCategory::Latest => write!(f, "Latest"),
			LightnovelCategory::Completed => write!(f, "Completed"),
			LightnovelCategory::Genre(genre) => write!(f, "Genre: {}", genre),
			LightnovelCategory::Title(title) => write!(f, "Title: {}", title),
		}
	}
}

impl LightnovelList {
	pub fn new(listtype: LightnovelCategory) -> Result<Self, Box<dyn Error>> {
		let client: Client = Config::new().set_base_url(Url::parse("https://readlightnovels.net")?).try_into()?;

		Ok(Self {
			client,
			category: listtype,
			page: 1,
			list: vec![],
		})
	}

	pub async fn scrape(&mut self) -> Result<bool, Box<dyn Error>> {
		use crate::scrape::lightnovel as lnScrape;
		use LightnovelCategory::*;

		let data = match &self.category {
			Latest => lnScrape::get_latest_ln(&self.client, self.page).await?,
			Completed => lnScrape::get_completed_ln(&self.client, self.page).await?,
			Genre(g) => lnScrape::get_genre_ln(&self.client, g, self.page).await?,
			Title(t) => lnScrape::get_title_ln(&self.client, t, self.page).await?,
		};

		match data {
			Some(mut data) => {
				self.list = data
					.iter_mut()
					.map(|(title, url)| Lightnovel::new(mem::take(title), mem::take(url)))
					.collect();

				self.page += 1;
				Ok(true)
			}
			None => Ok(false),
		}
	}

	pub async fn next_page(&mut self) -> Result<bool, Box<dyn Error>> {
		self.scrape().await
	}

	pub async fn perv_page(&mut self) -> Result<bool, Box<dyn Error>> {
		self.page -= 2;
		self.scrape().await
	}
}

impl Iterator for &mut LightnovelList {
	type Item = Lightnovel;

	fn next(&mut self) -> Option<Self::Item> {
		if self.list.is_empty() {
			return None;
		}
		Some(self.list.remove(0))
	}
}

impl Lightnovel {
	fn new(title: String, url: String) -> Self {
		Self {
			id: None,
			title,
			url,
			page: 1,
			last_page: None,
			chapters: vec![],
		}
	}

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_last_page(&self) -> Option<i32> {
		self.last_page
	}

	pub async fn scrape(&mut self) -> Result<bool, Box<dyn Error>> {
		use crate::scrape::chapter as chScrape;

		let data = match self.id {
			Some(id) => {
				let (_, _, data) = chScrape::get_chapters(Some(id), &self.url, Some(self.page)).await?;
				data
			}
			None => {
				let data: Option<Vec<(String, String)>>;
				(self.id, self.last_page, data) = chScrape::get_chapters(None, &self.url, Some(self.page)).await?;
				data
			}
		};

		match data {
			Some(mut data) => {
				self.chapters = data
					.iter_mut()
					.enumerate()
					.map(|(i, (title, url))| LightnovelChapter::new(mem::take(title), mem::take(url), (self.page - 1) * 48 + i as i32 + 1))
					.collect();

				self.page += 1;
				Ok(true)
			}
			None => Ok(false),
		}
	}

	pub async fn scrape_chapter_page(&mut self, page: i32) -> Result<bool, Box<dyn Error>> {
		self.page = page;
		self.scrape().await
	}
}

impl Iterator for &mut Lightnovel {
	type Item = LightnovelChapter;

	fn next(&mut self) -> Option<Self::Item> {
		if self.chapters.is_empty() {
			return None;
		}
		Some(self.chapters.remove(0))
	}
}

impl LightnovelChapter {
	fn new(title: String, url: String, chapter_number: i32) -> Self {
		Self {
			title,
			url,
			chapter_number,
			paragraph: vec![],
		}
	}

	pub fn get_title(&self) -> &String {
		&self.title
	}

	pub fn get_chapter_number(&self) -> i32 {
		self.chapter_number
	}

	pub fn len(&self) -> usize {
		self.paragraph.len()
	}

	pub async fn scrape(&mut self) -> Result<(), Box<dyn Error>> {
		use crate::scrape::paragraph as pScrape;

		let data = pScrape::get_paragraph(&self.url).await?;

		self.paragraph = data.iter().map(|p| p.clone()).collect();

		Ok(())
	}
}

impl<Idx> std::ops::Index<Idx> for LightnovelChapter
where
	Idx: std::slice::SliceIndex<[String]>,
{
	type Output = Idx::Output;

	fn index(&self, index: Idx) -> &Self::Output {
		&self.paragraph[index]
	}
}

impl Iterator for &mut LightnovelChapter {
	type Item = String;

	fn next(&mut self) -> Option<Self::Item> {
		if self.paragraph.is_empty() {
			return None;
		}
		Some(self.paragraph.remove(0))
	}
}
