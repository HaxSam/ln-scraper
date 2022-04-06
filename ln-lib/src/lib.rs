#![allow(dead_code)]
mod scrape;

use std::clone::Clone;
use std::error::Error;

use scrape::{chapter as chScrape, lightnovel as lnScrape, paragraph as pScrape};
use surf::{Client, Config, Url};

#[derive(Clone)]
pub struct Lightnovel {
	id: Option<i32>,
	title: String,
	url: String,
	genre: Vec<LightnovelCategory>,
	chapters: Vec<LightnovelChapter>,
	chapter_number: i32,
}

#[derive(Clone)]
pub struct LightnovelChapter {
	title: String,
	url: String,
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

impl Lightnovel {
	fn new(title: String, url: String, genre: Option<Vec<LightnovelCategory>>) -> Self {
		Self {
			id: None,
			title,
			url,
			genre: genre.unwrap_or(vec![]),
			chapters: vec![],
			chapter_number: 0,
		}
	}
	fn add_chapter(&mut self, chapter: LightnovelChapter) {
		self.chapters.push(chapter);
	}
	fn add_genre(&mut self, gendre: LightnovelCategory) {
		if let LightnovelCategory::Genre(g) = gendre {
			self.genre.push(LightnovelCategory::Genre(g));
		}
	}
	pub async fn scrape_chapter_page(&mut self, page: Option<i32>) -> Result<(), Box<dyn Error>> {
		let data = chScrape::get_chapters(self, page).await?;

		self.chapters = vec![];

		for (title, url) in data {
			self.add_chapter(LightnovelChapter::new(title, url));
		}

		Ok(())
	}
	pub fn get_lightnovel_chapter(&self, index: i32) -> LightnovelChapter {
		self.chapters.get(index as usize).unwrap().clone()
	}
	pub fn get_lightnovel_chapters(&self) -> Vec<LightnovelChapter> {
		self.chapters.clone()
	}
}

impl LightnovelChapter {
	fn new(title: String, url: String) -> Self {
		Self {
			title,
			url,
			paragraph: vec![],
		}
	}
	fn add_paragraph(&mut self, paragraph: String) {
		self.paragraph.push(paragraph);
	}
	pub async fn scrape_paragraph(&mut self) -> Result<(), Box<dyn Error>> {
		let data = pScrape::get_paragraph(&self).await?;

		self.paragraph = vec![];

		for paragraph in data {
			self.add_paragraph(paragraph);
		}

		Ok(())
	}
	pub fn get_lightnovel_paragraphs(&self) -> Vec<String> {
		self.paragraph.clone()
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
			page: 1,
			list: vec![],
		})
	}
	pub async fn scrape(&mut self) -> Result<(), Box<dyn Error>> {
		let data = match &self.category {
			LightnovelCategory::Latest => lnScrape::get_latest_ln(&self.client, self.page).await?,
			LightnovelCategory::Completed => lnScrape::get_completed_ln(&self.client, self.page).await?,
			LightnovelCategory::Genre(g) => lnScrape::get_genre_ln(&self.client, g, self.page).await?,
			LightnovelCategory::Title(t) => lnScrape::get_title_ln(&self.client, t, self.page).await?,
		};

		self.list = vec![];

		for (title, url) in data {
			self.list.push(Lightnovel::new(title, url, None));
		}

		Ok(())
	}
	pub async fn next_page(&mut self) -> Result<(), Box<dyn Error>> {
		self.page += 1;
		self.scrape().await
	}
	pub async fn perv_page(&mut self) -> Result<(), Box<dyn Error>> {
		self.page -= 1;
		self.scrape().await
	}
	pub async fn go_to_page(&mut self, page: i32) -> Result<(), Box<dyn Error>> {
		self.page = page;
		self.scrape().await
	}
	pub fn get_lightnovel(self, index: usize) -> Lightnovel {
		self.list.get(index).unwrap().clone()
	}
	pub fn get_lightnovels(&self) -> Vec<Lightnovel> {
		self.list.clone()
	}
}
