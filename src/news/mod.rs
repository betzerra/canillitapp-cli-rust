
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

mod models;
mod service;
mod formatter;

pub fn search(term: String) {

	// Percent encoding ("Macri Gato" to "Macri%20Gato")
	let encoded_search_term = utf8_percent_encode(&term, DEFAULT_ENCODE_SET);

	// Fetch search news
	let news = service::fetch_from_search(encoded_search_term.to_string()).unwrap();

	// Render
	for x in &news {
		formatter::print_news_with_highlight(&x, &term);
    println!("");
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
