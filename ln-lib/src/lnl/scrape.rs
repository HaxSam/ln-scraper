use scraper::{Html, Selector};

use crate::cnf::{CLIENT, LIGHTNOVEL_SITE};
use crate::err::Error;

pub async fn get_ln(url: String) -> Result<(Vec<(String, String)>, Option<usize>), Error> {
	let client = CLIENT.get().unwrap();

	let req = client.get(url);
	let mut res = req.send().await?;
	let body = res.body_string().await?;

	let document = Html::parse_document(&body);

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
