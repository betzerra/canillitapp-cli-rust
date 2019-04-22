
use url::percent_encoding::{utf8_percent_encode, DEFAULT_ENCODE_SET};

mod models;
mod service;
mod formatter;

pub fn search(term: String) {

	// Percent encoding ("Macri Gato" to "Macri%20Gato")
	let encoded_search_term = utf8_percent_encode(&term, DEFAULT_ENCODE_SET);

	// Render news
	let news = service::fetch_from_search(encoded_search_term.to_string()).unwrap();

	for x in &news {
		formatter::print_news(&x, &term);
    println!("");
	}
}
