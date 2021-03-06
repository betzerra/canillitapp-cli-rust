use colored::*;
use std::env;

mod news;

fn search(args: Vec<String>) {
	if args.len() <= 2 {
		return
	}

	/* 
	 *	Dropping first 2 arguments and merging all of them on a string.
	 *	First one is the executable path, second one is the command "search".
	 *
	 *	We're just interested on the rest of the arguments which are part of the
	 *	search we did.
	 */

	let mut search_arguments: Vec<String> = args.clone().drain(2..).collect();
	
	// If there's a -s flag, then format the results differently
	let short_format_flag = String::from("-s");
	let is_short_format = search_arguments.contains(&short_format_flag);
	if is_short_format {
		search_arguments.retain(|x| x != &short_format_flag);
	}

	let search_term = search_arguments.join(" ");
	news::search(search_term, is_short_format);
}

fn help() {
	println!("{} {}", "canillitapp-cli-rust".yellow(), "1.1.0");
	println!("{} {}", "Por", "@betzerra".purple());
	println!("{}", "https://github.com/betzerra/canillitapp-cli-rust".blue());

	let usage = r#"
Uso: canillitapp-cli [search <término>] [popular] [trending] [-s]
	"#;
	println!("{}", usage);
}

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() <= 1 {
		help();
		return
	}

  match args[1].as_ref() {
    "search" => search(args),
    "trending" => news::trending(),
    "popular" => news::popular(),
    _ => help()
  }
}
