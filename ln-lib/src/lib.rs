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

impl Lightnovel {
	fn new(title: String, url: String) -> Self {
		Self {
			id: None,
			title,
			url,
			chapters: vec![],
		}
	}
	pub fn get_title(&self) -> &String {
		&self.title
	}
	pub async fn scrape_chapter_page(&mut self, page: Option<i32>) -> Result<(), Box<dyn Error>> {
		let data = chScrape::get_chapters(self, page).await?;

		self.chapters = data
			.iter()
			.enumerate()
			.map(|(i, (title, url))| {
				LightnovelChapter::new(
					title.clone(),
					url.clone(),
					(page.unwrap_or(1) - 1) * 48 + i as i32 + 1,
				)
			})
			.collect();

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
	pub async fn scrape_paragraph(&mut self) -> Result<(), Box<dyn Error>> {
		let data = pScrape::get_paragraph(&self).await?;

		self.paragraph = data.iter().map(|p| p.clone()).collect();

		Ok(())
	}
	pub fn get_lightnovel_paragraphs(&self) -> &Vec<String> {
		&self.paragraph
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
		use LightnovelCategory::*;

		let data = match &self.category {
			Latest => lnScrape::get_latest_ln(&self.client, self.page).await?,
			Completed => lnScrape::get_completed_ln(&self.client, self.page).await?,
			Genre(g) => lnScrape::get_genre_ln(&self.client, g, self.page).await?,
			Title(t) => lnScrape::get_title_ln(&self.client, t, self.page).await?,
		};

		self.list = data
			.iter()
			.map(|(title, url)| Lightnovel::new(title.clone(), url.clone()))
			.collect();

		Ok(())
	}
	pub async fn go_to_page(&mut self, page: i32) -> Result<(), Box<dyn Error>> {
		self.page = page;
		self.scrape().await
	}
	pub async fn next_page(&mut self) -> Result<(), Box<dyn Error>> {
		self.go_to_page(self.page + 1).await
	}
	pub async fn perv_page(&mut self) -> Result<(), Box<dyn Error>> {
		self.go_to_page(self.page - 1).await
	}
	pub fn get_lightnovel(&self, index: usize) -> Lightnovel {
		self.list.get(index).unwrap().clone()
	}
	pub fn get_lightnovels(&self) -> &Vec<Lightnovel> {
		&self.list
	}
}
