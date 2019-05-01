use chrono::prelude::*;
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

mod models;
mod service;
mod formatter;
mod test;

pub fn search(term: String, short_format: bool) {

	// Percent encoding ("Macri Gato" to "Macri%20Gato")
	let encoded_search_term = utf8_percent_encode(&term, DEFAULT_ENCODE_SET);

	// Fetch search news
	let news = service::fetch_from_search(encoded_search_term.to_string()).unwrap();

	// Render
	for x in &news {
		formatter::print_news_with_highlight(&x, &term, short_format);
	}
}

pub fn popular() {

	// Fetch popular news
	let news = service::fetch_popular().unwrap();

	// Render
	for x in &news {
		formatter::print_news(&x);
		println!("");
	}
}

pub fn trending() {

	let date = Local::now().format("%Y-%m-%d");

	// Fetch popular news
	let news = service::fetch_trending(date.to_string()).unwrap();

	// Render
	for x in &news.keywords {
		formatter::print_array_of_news_with_highlight(&news.news[x], x);
		println!("");
	}
}
