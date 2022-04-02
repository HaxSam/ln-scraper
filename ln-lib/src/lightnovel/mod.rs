mod scrape;

use std::error::Error;
use surf::{Client, Config, Url};

pub struct Lightnovel {
	title: String,
	url: String,
	genre: Vec<LightnovelCategory>,
	chapter: Vec<Box<LightnovelChapter>>,
	chapter_number: i32,
}
pub struct LightnovelChapter {
	title: String,
	paragraph: Vec<String>,
}
pub struct LightnovelList {
	client: Client,
	category: LightnovelCategory,
	page: i32,
	list: Vec<Lightnovel>,
}
pub enum LightnovelCategory {
	Latest,
	Completed,
	Genre(String),
	Title(String),
}

impl Lightnovel {
	fn new(title: String, url: String, genre: Option<Vec<LightnovelCategory>>) -> Self {
		Self {
			title,
			url,
			genre: genre.unwrap_or(vec![]),
			chapter: vec![],
			chapter_number: 0,
		}
	}
	fn add_genre(&mut self, gendre: LightnovelCategory) {
		if let LightnovelCategory::Genre(g) = gendre {
			self.genre.push(LightnovelCategory::Genre(g));
		}
	}
	fn add_chapter(&mut self, chapter: LightnovelChapter) {
		self.chapter.push(Box::new(chapter));
	}
}

impl LightnovelChapter {
	fn new(title: String, paragraph: Option<Vec<String>>) -> Self {
		Self {
			title,
			paragraph: paragraph.unwrap_or(vec![]),
		}
	}
	fn add_paragraph(&mut self, paragraph: String) {
		self.paragraph.push(paragraph);
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
			LightnovelCategory::Latest => scrape::get_latest_ln(&self.client, self.page).await?,
			LightnovelCategory::Completed => scrape::get_completed_ln(&self.client, self.page).await?,
			LightnovelCategory::Genre(g) => scrape::get_genre_ln(&self.client, g, self.page).await?,
			LightnovelCategory::Title(t) => scrape::get_title_ln(&self.client, t, self.page).await?,
		};

		self.list = vec![];

		for (title, url) in data {
			self.list.push(Lightnovel::new(title, url, None));
		}

		Ok(())
	}
	pub async fn next_page(&mut self, page: Option<i32>) -> Result<(), Box<dyn Error>> {
		self.page = page.unwrap_or(self.page + 1);
		self.scrape().await
	}
}
