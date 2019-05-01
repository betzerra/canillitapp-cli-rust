use crate::news::models::News;
use colored::*;
use std::collections::HashSet;

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

pub fn print_news_with_highlight(news: &News, highlight: &String, short: bool) {
	
	if short {
		println!("- {}. {}", highlighted(&news.title, &highlight), news.source_name.purple());
		return
	}

	println!("- {}", highlighted(&news.title, &highlight));
	println!("  {} - {}", news.source_name.purple(), news.date_string());
	println!("  {}", news.website_url().blue());

	match news.reactions_string() {
	  Some(s) => println!("  {}", s),
	  None => ()
	}
	
	println!("");
}

fn reactions_from_array_of_news(news: &Vec<News>) -> Vec<String> {

	let mut set: HashSet<String> = HashSet::new();
	for i in news {
		for r in i.reactions_array() {
			if !set.contains(&r) {
				set.insert(r);
			}
		}
	}

	set.into_iter().collect::<Vec<String>>()
}

pub fn print_array_of_news_with_highlight(news: &Vec<News>, highlight: &String) {

	let n = news.first().unwrap();
	print!("- {}", highlight.green());

	let reactions = reactions_from_array_of_news(&news);
	if reactions.len() > 0 {
		let summary_reactions = reactions.join("");
		print!(" {}", summary_reactions);
	}

	println!(" {}", format!("({})", news.len()).green());
	println!("  {}", highlighted(&n.title, &highlight));
	println!("  {} - {}", n.source_name.purple(), n.date_string());
	println!("  {}", n.website_url().blue());
}