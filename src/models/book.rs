use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchResponse {
    pub docs: Vec<BookDoc>,
}

#[derive(Debug, Deserialize)]
pub struct BookDoc {
    pub title: Option<String>,
    pub author_name: Option<Vec<String>>,
    pub first_publish_year: Option<i32>,
}

