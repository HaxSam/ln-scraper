use std::error::Error;

use scraper::{Html, Selector};
use surf::Client;

fn get_title_href(body: String) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	let document = Html::parse_document(&body);

	let div_select = Selector::parse("div.home-truyendecu").unwrap();
	let a_select = Selector::parse("a").unwrap();

	let mut result = Vec::new();

	document.select(&div_select).for_each(|div| {
		div.select(&a_select).for_each(|a| {
			let href = a.value().attr("href").unwrap();
			let title = a.value().attr("title").unwrap();

			result.push((title.to_string(), href.to_string()));
		});
	});

	if result.is_empty() {
		return Ok(None);
	}

	Ok(Some(result))
}

pub async fn get_latest_ln(
	client: &Client, page: i32,
) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	let req = client.get(format!("/latest/page/{}", page));
	let client = surf::client().with(surf::middleware::Redirect::new(3));
	let mut res = client.send(req).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}

pub async fn get_completed_ln(
	client: &Client, page: i32,
) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	let req = client.get(format!("/completed/page/{}", page));
	let client = surf::client().with(surf::middleware::Redirect::new(3));
	let mut res = client.send(req).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}

pub async fn get_genre_ln(
	client: &Client, genre: &String, page: i32,
) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	let req = client.get(format!("/{}/page/{}", genre, page));
	let client = surf::client().with(surf::middleware::Redirect::new(3));
	let mut res = client.send(req).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}

pub async fn get_title_ln(
	client: &Client, title: &String, page: i32,
) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
	let req = client.get(format!("/page/{}?s={}", page, title));
	let client = surf::client().with(surf::middleware::Redirect::new(3));
	let mut res = client.send(req).await?;
	let res_body = res.body_string().await?;

	get_title_href(res_body)
}
