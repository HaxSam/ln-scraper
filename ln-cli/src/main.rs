use std::error::Error;

use ln_lib::lightnovel::{LightnovelCategory, LightnovelList};
use surf::{Client, Config, Url};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let client: Client = Config::new()
		.set_base_url(Url::parse("https://readlightnovels.net")?)
		.try_into()?;

	let mut list = LightnovelList::new(LightnovelCategory::Latest, client);
	list.scrape().await?;

	Ok(())
}
