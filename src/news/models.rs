use chrono::prelude::*;
use serde::{Deserialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Trending {
  pub keywords: Vec<String>,
  pub news: HashMap<String, Vec<News>>,
}

#[derive(Deserialize, Debug)]
pub struct News {
  pub news_id: i32,
  pub url: String,
  pub title: String,
  pub date: i32,
  source_id: i32,
  pub source_name: String,
  img_url: Option<String>,
  reactions_count: Option<i32>,
  content_views_count: Option<i32>,
  category: Option<String>,
  reactions: Vec<ReactionSummary>
}

#[derive(Deserialize, Debug)]
pub struct ReactionSummary {
  reaction: String,
  amount: i32,
  date: i32
}

impl News {

  pub fn date_string(&self) -> String {

    let datetime = NaiveDateTime::from_timestamp(self.date as i64, 0);
    let timestamp_str = datetime.format("%d-%m-%Y %H:%M");
    timestamp_str.to_string()
  }

  pub fn reactions_array(&self) -> Vec<String> {

    self.reactions
        .iter()
        .map( |x| x.reaction.to_string() )
        .collect::<Vec<_>>()
  }

  pub fn reactions_string(&self) -> Option<String> {

    if self.reactions.len() == 0 {
      return None
    }

    let tmp = self.reactions
          .iter()
          .map( |x| x.to_string())
          .collect::<Vec<_>>();

    Some(tmp.join(" - "))
  }

  pub fn website_url(&self) -> String {

    format!("https://canillitapp.com/article/{}", self.news_id)
  }
}

impl ReactionSummary {

  pub fn to_string(&self) -> String {

    format!("{} {}", self.reaction, self.amount)
  }
}
