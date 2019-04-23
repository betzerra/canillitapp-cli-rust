use crate::news::models::News;
use reqwest;

struct Config {
  base_url: &'static str
}

static CONFIG: Config = Config {
  base_url: "https://api.canillitapp.com"
};

// TODO: Implement this on unit tests, maybe?
#[allow(dead_code)]
pub fn fetch_from_id(id: i32) -> Result<News, reqwest::Error> {

  let request_url = format!("{base_url}/news/{news_id}",
                              base_url = CONFIG.base_url,
                              news_id = id);

  let mut response = reqwest::get(&request_url)?;

  let news: News = response.json()?;
  Ok(news)
}

// TODO: Implement fetch news from date
#[allow(dead_code)]
pub fn fetch_from_date(date: String) -> Result<Vec<News>, reqwest::Error> {

  let request_url = format!("{base_url}/latest/{date}",
                            base_url = CONFIG.base_url,
                            date = date);

  let mut response = reqwest::get(&request_url)?;

  let news: Vec<News> = response.json()?;
  Ok(news)
}

pub fn fetch_from_search(search: String) -> Result<Vec<News>, reqwest::Error> {

  let request_url = format!("{base_url}/search/{search}",
                            base_url = CONFIG.base_url,
                            search = search);

  let mut response = reqwest::get(&request_url)?;

  let news: Vec<News> = response.json()?;
  Ok(news) 
}

pub fn fetch_popular() -> Result<Vec<News>, reqwest::Error> {

  let request_url = format!("{base_url}/popular",
                            base_url = CONFIG.base_url);

  let mut response = reqwest::get(&request_url)?;

  let news: Vec<News> = response.json()?;
  Ok(news) 
}
