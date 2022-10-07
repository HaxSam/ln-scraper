use error_stack::{Report, Result};
use scraper::{Html, Selector};

use crate::err::{ChapterError, SurfError};

pub async fn get_paragraph(url: &String) -> Result<Vec<String>, ChapterError> {
	let mut res = match surf::get(url).send().await {
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
	let p_selector = Selector::parse("div.chapter-content>p").unwrap();

	let result = document
		.select(&p_selector)
		.map(|p| p.text().collect::<Vec<_>>().join("\n"))
		.collect::<Vec<_>>();

	Ok(result)
}
