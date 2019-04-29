use crate::news::*;

#[cfg(test)]
mod tests {
  // Importing names from outer (for mod tests) scope.
  use super::*;

  #[test]
  fn test_fetch_from_id() {

    let id = 1;
    let _news = match service::fetch_from_id(id) {
      Ok(n) => n,
      Err(error) => {
        panic!("There was a problem fetching news_from_id {}: {:?}", id, error)
      },
    };
  }

  #[test]
  fn test_fetch_from_date() {

    let date = "2019-01-01";
    let _news = match service::fetch_from_date(date.to_string()) {
      Ok(n) => n,
      Err(error) => {
        panic!("There was a problem fetching fetch_from_date {}: {:?}", date, error)
      },
    };
  }

  #[test]
  fn test_fetch_from_search() {

    let term = "Mauricio Macri";
    let _news = match service::fetch_from_search(term.to_string()) {
      Ok(n) => n,
      Err(error) => {
        panic!("There was a problem fetching fetch_from_search {}: {:?}", term, error)
      },
    }; 
  }

  #[test]
  fn test_fetch_popular() {

    let _news = match service::fetch_popular() {
      Ok(n) => n,
      Err(error) => {
        panic!("There was a problem fetching fetch_popular: {:?}", error)
      },
    };
  }

  #[test]
  fn test_fetch_trending() {

    let date = "2019-01-01";
    let _news = match service::fetch_trending(date.to_string()) {
      Ok(n) => n,
      Err(error) => {
        panic!("There was a problem fetching fetch_trending {}: {:?}", date, error)
      },
    };
  }
}