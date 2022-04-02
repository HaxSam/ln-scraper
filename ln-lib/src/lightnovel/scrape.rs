use scraper::{Html, Selector};
use std::error::Error;
use surf::Client;

fn get_title_href(body: &String) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let now = std::time::Instant::now();
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
	print!("{}", now.elapsed().as_millis());
	Ok(result)
}

pub async fn get_latest_ln(
	client: &Client, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = if page == 1 {
		"/latest".to_string()
	} else {
		format!("/latest/page/{}", page)
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(&res_body)
}

pub async fn get_completed_ln(
	client: &Client, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = if page == 1 {
		"/completed".to_string()
	} else {
		format!("/completed/page/{}", page)
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(&res_body)
}

pub async fn get_genre_ln(
	client: &Client, genre: &String, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = if page == 1 {
		format!("/{}", genre)
	} else {
		format!("/{}/page/{}", genre, page)
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(&res_body)
}

pub async fn get_title_ln(
	client: &Client, title: &String, page: i32,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
	let path = if page == 1 {
		format!("/?s={}", title)
	} else {
		format!("/page/{}?s={}", page, title)
	};

	let mut res = client.get(path).await?;
	let res_body = res.body_string().await?;

	get_title_href(&res_body)
}
