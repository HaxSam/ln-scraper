use std::error::Error;

use ln_lib::lightnovel::{LightnovelCategory, LightnovelList};
use surf::{Client, Config, Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut list = LightnovelList::new(LightnovelCategory::Genre(String::from("actions")))?;
	list.scrape().await?;
	list.print_list();
	list.next_page(None).await?;
	list.print_list();

	Ok(())
}
