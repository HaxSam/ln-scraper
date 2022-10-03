use scraper::{Html, Selector};
use surf::http::convert::{Deserialize, Serialize};

use crate::cnf::CLIENT;
use crate::err::Error;

#[derive(Deserialize, Serialize)]
struct ChapterResponse {
	list_chap: String,
	pagination: String,
}

fn get_id(document: &Html) -> Option<usize> {
	let id_selector = Selector::parse("input#id_post").unwrap();
	let first_element = document.select(&id_selector).next();

	match first_element {
		Some(element) => {
			let id = element.value().attr("value").unwrap();
			Some(id.parse::<usize>().unwrap())
		}
		None => None,
	}
}

fn get_last_page(document: &Html) -> Option<usize> {
	let page_select = Selector::parse("a[data-page]").unwrap();

	let last_page = document.select(&page_select).last();

	match last_page {
		Some(element) => {
			let last_page = element.value().attr("data-page").unwrap();
			Some(last_page.parse().unwrap())
		}
		None => None,
	}
}

pub async fn get_cha(url: &String, page: Option<usize>) -> Result<(usize, Option<usize>, Vec<(String, String)>), Error> {
	let client = CLIENT.get().unwrap();

	let mut res = client.get(url).send().await?;
	let res_body = res.body_string().await?;

	let document = Html::parse_document(&res_body);

	let id = match get_id(&document) {
		Some(id) => id,
		None => return Err(Error::GetIdError),
	};

	let last_page = get_last_page(&document);

	let chapters = match page {
		Some(1) | None => parse_html(&document),
		Some(p) => get_cha_by_id(id, p).await?,
	};

	Ok((id, last_page, chapters))
}

pub async fn get_cha_by_id(id: usize, page: usize) -> Result<Vec<(String, String)>, Error> {
	let client = CLIENT.get().unwrap();

	let mut res = client
		.post("/wp-admin/admin-ajax.php")
		.header("content-type", "application/x-www-form-urlencoded")
		.body_string(format!("action=tw_ajax&type=pagination&id={}&page={}", id, page))
		.await?;

	let ChapterResponse { list_chap, pagination: _ } = res.body_json().await?;

	let document_fragment = Html::parse_fragment(&list_chap);

	Ok(parse_html(&document_fragment))
}

fn parse_html(document: &Html) -> Vec<(String, String)> {
	let chapter_selector = Selector::parse("ul.list-chapter>li>a").unwrap();

	document
		.select(&chapter_selector)
		.map(|a| {
			let href = a.value().attr("href").unwrap();
			let title = a.text().collect::<Vec<_>>().join(" ");
			(title.to_string(), href.to_string())
		})
		.collect::<Vec<(String, String)>>()
}
