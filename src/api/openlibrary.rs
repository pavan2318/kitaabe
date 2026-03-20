use crate::error::KitaabeError;
use crate::models::book::SearchResponse;

pub async fn search_books(query: &str) -> Result<SearchResponse, KitaabeError> {
let encoded = urlencoding::encode(query);
let url = format!("https://openlibrary.org/search.json?q={}",
    encoded
    );

let res = reqwest::get(&url).await?;
let data = res.json::<SearchResponse>().await?;

Ok(data)

}
