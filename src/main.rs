use std::io::Cursor;

async fn scrape() -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.boursier.com/indices/composition/cac-40-FR0003500008,FR.html/").await?;
    let body = response.text().await?;
    let cursor = Cursor::new(body);
    let document = select::document::Document::from_read(cursor).unwrap();
    let table = document.find(select::predicate::Class("table table--values table--no-auto")).next();
    if let Some(table) = table {
        for row in table.find(select::predicate::Name("tr")) {
            let cells: Vec<_> = row.find(select::predicate::Name("td"))
                .map(|cell| cell.text().trim().to_string())
                .collect();
            println!("{:?}", cells);
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    scrape().await?;
    Ok(())
}


