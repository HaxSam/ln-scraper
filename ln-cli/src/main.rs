mod menu;

use menu::{show_chapters, show_ln};

use std::error::Error;

use std::cmp::{max, min};
use tuikit::attr::*;
use tuikit::event::{Event, Key};
use tuikit::term::{Term, TermHeight};

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

	let mut chapter = show_chapters(&mut ln).await?.unwrap();
	chapter.scrape().await?;

	let term: Term<()> = Term::with_height(TermHeight::Percent(100)).unwrap();
	let mut line = 1;
	let mut lineselect: i32 = 0;
	let col = 0;

	let _ = term.present();

	while let Ok(ev) = term.poll_event() {
		let _ = term.clear();

		let (_width, height) = term.term_size().unwrap();

		match ev {
			Event::Key(Key::ESC) | Event::Key(Key::Char('q')) => break,
			Event::Key(Key::Up) => lineselect = max(lineselect - 1, -1),
			Event::Key(Key::Down) => lineselect = min(lineselect + 1, height as i32 + 1),
			_ => {}
		}

		line = if lineselect == -1 {
			line - 1
		} else if lineselect == height as i32 + 1 {
			line + 1
		} else {
			line
		};
		lineselect = if lineselect == -1 {
			0
		} else if lineselect == height as i32 + 1 {
			height as i32
		} else {
			lineselect
		};

		let text = &chapter[line - 1..min(height + line - 1, chapter.len() - 1)];

		let attr = Attr {
			fg: Color::RED,
			..Attr::default()
		};

		let mut row = 0;
		for line in text {
			let _ = term.print_with_attr(row, col, line, attr);
			row += 1
		}
		let _ = term.set_cursor(lineselect as usize, col);
		let _ = term.present();
	}

	Ok(())
}
