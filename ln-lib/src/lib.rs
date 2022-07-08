mod scrape;

use std::clone::Clone;
use std::error::Error;
use std::fmt;
use std::iter::Iterator;

use scrape::{chapter as chScrape, lightnovel as lnScrape, paragraph as pScrape};
use surf::{Client, Config, Url};

#[derive(Clone)]
pub struct Lightnovel {
	id: Option<i32>,
	title: String,
	url: String,
	page: i32,
	last_page: Option<i32>,
	chapters: Vec<LightnovelChapter>,
}

#[derive(Clone)]
pub struct LightnovelChapter {
	title: String,
	url: String,
	chapter_number: i32,
	paragraph: Vec<String>,
}

pub struct LightnovelList {
	client: Client,
	category: LightnovelCategory,
	page: i32,
	list: Vec<Lightnovel>,
}

#[derive(Clone)]
pub enum LightnovelCategory {
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
		let client: Client = Config::new()
			.set_base_url(Url::parse("https://readlightnovels.net")?)
			.try_into()?;

		Ok(Self {
			client,
			category: listtype,
			page: 0,
			list: vec![],
		})
	}

	pub async fn scrape(&mut self) -> Result<bool, Box<dyn Error>> {
		use LightnovelCategory::*;

		self.page += 1;

		let data = match &self.category {
			Latest => lnScrape::get_latest_ln(&self.client, self.page).await?,
			Completed => lnScrape::get_completed_ln(&self.client, self.page).await?,
			Genre(g) => lnScrape::get_genre_ln(&self.client, g, self.page).await?,
			Title(t) => lnScrape::get_title_ln(&self.client, t, self.page).await?,
		};

		match data {
			Some(data) => {
				self.list = data
					.iter()
					.map(|(title, url)| Lightnovel::new(title.clone(), url.clone()))
					.collect();
				Ok(true)
			}
			None => {
				self.page -= 1;
				Ok(false)
			}
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
		let data = chScrape::get_chapters(self, Some(self.page)).await?;

		self.chapters = vec![];

		match data {
			Some(data) => {
				self.chapters = data
					.iter()
					.enumerate()
					.map(|(i, (title, url))| {
						LightnovelChapter::new(
							title.clone(),
							url.clone(),
							(self.page - 1) * 48 + i as i32 + 1,
						)
					})
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

	pub async fn scrape(&mut self) -> Result<(), Box<dyn Error>> {
		let data = pScrape::get_paragraph(&self).await?;

		self.paragraph = data.iter().map(|p| p.clone()).collect();

		Ok(())
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
