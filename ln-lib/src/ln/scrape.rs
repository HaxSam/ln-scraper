use error_stack::{Report, Result};
use scraper::{Html, Selector};
use surf::http::convert::{Deserialize, Serialize};

use crate::cfg::CLIENT;
use crate::err::{LightnovelError, SurfError};

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

pub async fn get_cha(url: &String, page: Option<usize>) -> Result<(usize, Option<usize>, Vec<(String, String)>), LightnovelError> {
	let client = CLIENT.get().unwrap();

	let req = client.get(url);
	let mut res = match req.send().await {
		Ok(res) => res,
		Err(_) => {
			let msg = format!("There was a problem while with sending the requet to: {}", url);
			let report = Report::new(SurfError::RequestError(msg.clone()).into());
			return Err(report.attach_printable(msg.clone()));
		}
	};
	let res_body = match res.body_string().await {
		Ok(body) => body,
		Err(_) => {
			let msg = format!("There was a problem with getting the body from: {}", url);
			let report = Report::new(SurfError::BodyParseError(msg.clone()).into());
			return Err(report.attach_printable(msg.clone()));
		}
	};

	let document = Html::parse_document(&res_body);

	let id = match get_id(&document) {
		Some(id) => id,
		None => {
			let msg = format!("There was a problem with getting the id from: {}", url);
			let report = Report::new(LightnovelError::GetIDError);
			return Err(report.attach_printable(msg.clone()));
		}
	};

	let last_page = get_last_page(&document);

	let chapters = match page {
		Some(1) | None => parse_html(&document),
		Some(p) => get_cha_by_id(id, p).await?,
	};

	Ok((id, last_page, chapters))
}

pub async fn get_cha_by_id(id: usize, page: usize) -> Result<Vec<(String, String)>, LightnovelError> {
	let client = CLIENT.get().unwrap();

	let mut res = match client
		.post("/wp-admin/admin-ajax.php")
		.header("content-type", "application/x-www-form-urlencoded")
		.body_string(format!("action=tw_ajax&type=pagination&id={}&page={}", id, page))
		.await
	{
		Ok(res) => res,
		Err(_) => {
			let msg = format!(
				"There was a problem while with sending the requet to /wp-admin/admin-ajax.php with the id: {}",
				id
			);
			let report = Report::new(SurfError::RequestError(msg.clone()).into());
			return Err(report.attach_printable(msg.clone()));
		}
	};

	let ChapterResponse { list_chap, pagination: _ } = match res.body_json().await {
		Ok(body) => body,
		Err(_) => {
			let msg = format!(
				"There was a problem with getting the json body from /wp-admin/admin-ajax.php with the id: {}",
				id
			);
			let report = Report::new(SurfError::BodyParseError(msg.clone()).into());
			return Err(report.attach_printable(msg.clone()));
		}
	};

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
			(title.trim().to_string(), href.to_string())
		})
		.collect::<Vec<(String, String)>>()
}
