use std::io::Cursor;

async fn scrape() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://cryptonews.com/").await?;
    let body = response.text().await?;
    let cursor = Cursor::new(body);
    let document = select::document::Document::from_read(cursor).unwrap();
    for node in document.find(select::predicate::Name("a")) {
        println!("Link: {}", node.attr("href").unwrap());
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    scrape().await?;
    Ok(())
}


