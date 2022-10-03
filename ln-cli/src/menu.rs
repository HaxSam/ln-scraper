use std::{error::Error, mem, thread};

use ln_lib::{Lightnovel, LightnovelChapter, LightnovelList};
use skim::prelude::*;

struct LightnovelWrapper {
	pub ln: Lightnovel,
}

struct LightnovelChapterWarpper {
	pub chapter: LightnovelChapter,
}

impl SkimItem for LightnovelWrapper {
	fn text(&self) -> Cow<str> {
		Cow::Borrowed(self.ln.get_title())
	}
}

impl SkimItem for LightnovelChapterWarpper {
	fn text(&self) -> Cow<str> {
		Cow::Borrowed(self.chapter.get_title())
	}
}

pub fn show_ln(list: &LightnovelList) -> Option<Lightnovel> {
	let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) = unbounded();

	let options = SkimOptionsBuilder::default()
		.height(Some("100%"))
		.prompt(Some("Select a lightnovel> "))
		.reverse(true)
		.build()
		.unwrap();

	for mut ln in list {
		let wrapper = LightnovelWrapper { ln: mem::take(&mut ln) };
		tx_item.send(Arc::new(wrapper)).unwrap();
	}

	drop(tx_item);

	let selected_itemes = Skim::run_with(&options, Some(rx_item))
		.map(|out| out.selected_items)
		.unwrap_or_else(|| Vec::new());

	let ln_wrapper_pointer = selected_itemes.into_iter().next();

	match ln_wrapper_pointer {
		Some(ln_wrapper_pointer) => {
			let ln_wrapper = (*ln_wrapper_pointer).as_any().downcast_ref::<LightnovelWrapper>().unwrap();
			Some(ln_wrapper.ln.clone())
		}
		None => None,
	}
}

pub async fn show_chapters(ln: &mut Lightnovel) -> Result<Option<LightnovelChapter>, Box<dyn Error>> {
	let (tx_chapter, rx_chapter): (SkimItemSender, SkimItemReceiver) = unbounded();

	let (tx, rx): (Sender<&str>, Receiver<&str>) = bounded(1);

	let handle = thread::spawn(move || {
		let options = SkimOptionsBuilder::default()
			.height(Some("100%"))
			.reverse(true)
			.prompt(Some("Select chapter> "))
			.multi(true)
			.build()
			.unwrap();

		let ret = Skim::run_with(&options, Some(rx_chapter))
			.map(|out| out.selected_items)
			.unwrap_or_else(|| Vec::new());

		tx.send("done").unwrap();

		ret
	});

	loop {
		for mut ch in ln.clone() {
			let wrapper = LightnovelChapterWarpper { chapter: mem::take(&mut ch) };

			if rx.try_recv().is_ok() {
				break;
			}

			tx_chapter.send(Arc::new(wrapper)).unwrap();
		}
		let got_chapters = ln.next_scrape().await?;
		if !got_chapters || rx.try_recv().is_ok() {
			if !got_chapters {
				for mut ch in ln.clone() {
					let wrapper = LightnovelChapterWarpper { chapter: mem::take(&mut ch) };

					if rx.try_recv().is_ok() {
						break;
					}

					tx_chapter.send(Arc::new(wrapper)).unwrap();
				}
			}
			break;
		}
	}

	drop(tx_chapter);

	let selected_itemes = handle.join().unwrap();
	let chapter_wrapper_pointer = selected_itemes.into_iter().next();

	let chapter = match chapter_wrapper_pointer {
		Some(chapter_wrapper_pointer) => {
			let chapter_wrapper = (*chapter_wrapper_pointer).as_any().downcast_ref::<LightnovelChapterWarpper>().unwrap();
			Some(chapter_wrapper.chapter.clone())
		}
		None => None,
	};

	Ok(chapter)
}
