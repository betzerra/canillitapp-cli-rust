use reqwest;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
struct News {
  news_id: i32,
  url: String,
  title: String,
  date: i32,
  source_id: i32,
  source_name: String,
  img_url: Option<String>,
  reactions_count: i32,
  content_views_count: i32,
  category: Option<String>
}

fn request_news(id: i32) -> Result<News, reqwest::Error> {

  let canillitapp_base_url = "https://api.canillitapp.com";
  let request_url = format!("{base_url}/news/{news_id}",
                              base_url = canillitapp_base_url,
                              news_id = id);

  let mut response = reqwest::get(&request_url)?;

  let news: News = response.json()?;
  Ok(news)
}

fn main() {
  let news = request_news(1).unwrap();
  print!("{:#?}", news);
}