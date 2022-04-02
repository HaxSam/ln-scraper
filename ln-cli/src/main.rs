use std::error::Error;

use ln_lib::{LightnovelCategory, LightnovelList};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let mut list = LightnovelList::new(LightnovelCategory::Genre(String::from("action")))?;
	list.scrape().await?;

	Ok(())
}
