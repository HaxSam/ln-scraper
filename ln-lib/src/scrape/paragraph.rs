use std::error::Error;

use scraper::{Html, Selector};

pub async fn get_paragraph(url: &String) -> Result<Vec<String>, Box<dyn Error>> {
	let mut res = surf::get(url).await?;
	let res_body = res.body_string().await?;

	let document = Html::parse_document(&res_body);
	let p_selector = Selector::parse("div.chapter-content>p").unwrap();

	let result = document
		.select(&p_selector)
		.map(|p| p.text().collect::<Vec<_>>().join("\n"))
		.collect::<Vec<_>>();

	Ok(result)
}
