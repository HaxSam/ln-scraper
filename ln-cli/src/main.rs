mod menu;

use menu::{show_chapters, show_ln};

use std::error::Error;

extern crate skim;
use clap::{AppSettings, ArgGroup, Parser};
use ln_lib::{LightnovelCategory, LightnovelList};

#[derive(Parser)]
#[clap(author, version, about)]
#[clap(global_setting(AppSettings::DeriveDisplayOrder))]
#[clap(group(ArgGroup::new("type").required(true).args(&["name", "genre", "completed", "latest"])))]
struct Args {
	/// search for the lightnovel you want to read
	name: Option<String>,

	/// get the lightnovels with the genre you want to read
	#[clap(short)]
	genre: Option<String>,

	/// get all completed lightnovels
	#[clap(short)]
	completed: bool,

	/// get the latest lightnovels
	#[clap(short)]
	latest: bool,

	/// get staged lightnovel
	#[clap(short)]
	stage: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();

	let category = if let Some(name) = args.name {
		LightnovelCategory::Title(name)
	} else if let Some(genre) = args.genre {
		LightnovelCategory::Genre(genre)
	} else if args.completed {
		LightnovelCategory::Completed
	} else if args.latest {
		LightnovelCategory::Latest
	} else {
		LightnovelCategory::Latest
	};

	let mut list = LightnovelList::new(category)?;
	list.scrape().await?;

	let mut ln = show_ln(&mut list).unwrap();

	ln.scrape().await?;

	let _chapter = show_chapters(&mut ln).await?;

	Ok(())
}
