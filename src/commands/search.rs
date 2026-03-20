use crate::api::openlibrary::search_books;
use crate::error::KitaabeError;

pub async fn run_search(query: &str) -> Result<(), KitaabeError> {
    let response = search_books(query).await?;

    println!("\n Results for: \"{}\"\n", query);

    for(i, book) in response.docs.iter().take(5).enumerate() {
        let title = book.title.as_deref().unwrap_or("Unknown Title");
        
        let author = book
            .author_name
            .as_ref()
            .and_then(|a| a.get(0))
            .map(|s| s.as_str())
            .unwrap_or("Unknown Author");

        let year = book
            .first_publish_year
            .map(|y| y.to_string())
            .unwrap_or_else(|| "N/A".to_string());

        println!(
            "{}, {}\n   {}\n    {}\n",
            i + 1,
            title,
            author,
            year
            );
    }
    Ok(())
    
}
