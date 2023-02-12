use error_stack::{Report, Result};
use scraper::{Html, Selector};

use crate::cfg::{CLIENT, LIGHTNOVEL_SITE};
use crate::err::{ListError, SurfError};

pub async fn get_ln(url: &String) -> Result<(Vec<(String, String)>, Option<usize>), ListError> {
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

	let ln_select = Selector::parse("div.home-truyendecu>a").unwrap();
	let page_select = Selector::parse("a[data-page]").unwrap();

	let result: Vec<(String, String)> = document
		.select(&ln_select)
		.map(|a| {
			let href = a.value().attr("href").unwrap();
			let title = a.value().attr("title").unwrap();

			(title.to_string(), href.to_string().replace(LIGHTNOVEL_SITE, ""))
		})
		.collect();

	let last_page = document.select(&page_select).last();

	match last_page {
		Some(a) => {
			let page = a.value().attr("data-page").unwrap();

			Ok((result, Some(page.parse().unwrap())))
		}
		None => Ok((result, None)),
	}
}
