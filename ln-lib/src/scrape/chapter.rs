use std::error::Error;

use crate::Lightnovel;
use scraper::{Html, Selector};
use surf::http::convert::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct ChapterResponse {
	list_chap: String,
	pagination: String,
}

fn get_id(document: &Html) -> Result<Option<i32>, Box<dyn Error>> {
	let id_selector = Selector::parse("input#id_post").unwrap();
	let first_element = document.select(&id_selector).next();

	match first_element {
		Some(element) => {
			let id = element.value().attr("value").unwrap();
			Ok(Some(id.parse::<i32>().unwrap()))
		}
		None => Ok(None),
	}
}

fn get_last_page(document: &Html) -> Result<Option<i32>, Box<dyn Error>> {
	let last_page_selector = Selector::parse("a[data-page]").unwrap();
	let last_page_element = document.select(&last_page_selector).last();

	match last_page_element {
		Some(element) => {
			let last_page = element.value().attr("data-page").unwrap();
			Ok(Some(last_page.parse::<i32>().unwrap()))
		}
		None => Ok(None),
	}
}

pub async fn get_chapters(
	ln: &mut Lightnovel, page: Option<i32>,
) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	match ln.id {
		None => {
			let req = surf::get(ln.url.clone());
			let client = surf::client().with(surf::middleware::Redirect::new(3));
			let mut res = client.send(req).await?;
			let res_body = res.body_string().await?;

			let document = Html::parse_document(&res_body);

			ln.id = get_id(&document)?;
			ln.last_page = get_last_page(&document)?;

			match page {
				Some(1) | None => scrape_chapters(&document).await,
				Some(p) => scrape_chapters_id(ln.id.unwrap(), p).await,
			}
		}
		Some(id) => scrape_chapters_id(id, page.unwrap_or(1)).await,
	}
}

async fn scrape_chapters_id(
	id: i32, page: i32,
) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
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

async fn scrape_chapters(document: &Html) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	let chapter_selector = Selector::parse("ul.list-chapter>li>a").unwrap();

	let mut result = Vec::new();

	document.select(&chapter_selector).for_each(|a| {
		let href = a.value().attr("href").unwrap();
		let title = a.value().attr("title").unwrap();

		result.push((title.to_string(), href.to_string()));
	});

	if result.is_empty() {
		return Ok(None);
	}

	Ok(Some(result))
}
