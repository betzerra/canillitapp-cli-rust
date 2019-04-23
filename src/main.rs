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

	let search_arguments: Vec<String> = args.clone().drain(2..).collect();
	let search_term = search_arguments.join(" ");
	news::search(search_term);
}

fn help() {
	println!("{} {}", "Canillitapp-rust-CLI".yellow(), "1.0");
	println!("{} {}", "Por", "@betzerra".purple());
	println!("{}", "https://github.com/betzerra".blue());

	let usage = r#"
Uso: canillitapp-cli [search <término>]
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
    _ => help()
  }
}
