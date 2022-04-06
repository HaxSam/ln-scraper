use std::error::Error;

use crate::Lightnovel;
use scraper::{Html, Selector};
use surf::http::convert::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ChapterResponse {
	list_chap: String,
	pagination: String,
}

fn get_id(document: &Html) -> Result<i32, Box<dyn Error>> {
	let id_selector = Selector::parse("input#id_post").unwrap();
	let first_element = document.select(&id_selector).next().unwrap();

	let id = first_element.value().attr("value").unwrap();

	Ok(id.parse::<i32>()?)
}

pub async fn get_chapters(
	ln: &mut Lightnovel, page: Option<i32>,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	match ln.id {
		None => {
			let mut res = surf::get(ln.url.clone()).await?;
			let res_body = res.body_string().await?;

			let document = Html::parse_document(&res_body);

			ln.id = Some(get_id(&document)?);

			match page {
				Some(1) | None => scrape_chapters(&document).await,
				Some(p) => scrape_chapters_id(ln.id.unwrap(), p).await,
			}
		}
		Some(id) => scrape_chapters_id(id, page.unwrap_or(1)).await,
	}
}

async fn scrape_chapters_id(id: i32, page: i32) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let mut res = surf::post("https://readlightnovels.net/wp-admin/admin-ajax.php")
		.header("content-type", "application/x-www-form-urlencoded")
		.body_string(format!(
			"action=tw_ajax&type=pagination&id={}&page={}",
			id, page
		))
		.await?;

	let ChapterResponse {
		list_chap,
		pagination: _,
	} = res.body_json().await?;

	let document_fragment = Html::parse_fragment(&list_chap);

	scrape_chapters(&document_fragment).await
}

async fn scrape_chapters(document: &Html) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let chapter_selector = Selector::parse("ul.list-chapter>li>a").unwrap();

	let mut result = Vec::new();

	document.select(&chapter_selector).for_each(|a| {
		let href = a.value().attr("href").unwrap();
		let title = a.value().attr("title").unwrap();

		result.push((title.to_string(), href.to_string()));
	});

	Ok(result)
}
