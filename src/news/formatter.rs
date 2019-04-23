use crate::news::models::News;

use colored::*;

fn highlighted(string: &String, highlighted: &String) -> String {

	let tmp = string.clone();
	let pos = tmp.to_lowercase().find(&highlighted.to_lowercase());

	if pos.is_some() {
		let p = pos.unwrap();

		let lead = &tmp[..p];
		let highlighted = highlighted.yellow().italic();
		let trail = &tmp[p + highlighted.len()..];
		
		return format!("{}{}{}", lead, highlighted, trail);
	}

	return tmp
}

pub fn print_news(news: &News) {
	println!("- {}", &news.title);
	println!("  {} - {}", news.source_name.purple(), news.date_string());
	println!("  {}", news.website_url().blue());

	match news.reactions_string() {
	  Some(s) => println!("  {}", s),
	  None => ()
	}
}

pub fn print_news_with_highlight(news: &News, highlight: &String) {
	
	println!("- {}", highlighted(&news.title, &highlight));
	println!("  {} - {}", news.source_name.purple(), news.date_string());
	println!("  {}", news.website_url().blue());

	match news.reactions_string() {
	  Some(s) => println!("  {}", s),
	  None => ()
	}
}