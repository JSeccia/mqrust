use std::io::Cursor;

struct Row {
    name: String,
    rate: String,
    variation: String,
    high: String,
    opening: String,
    low: String,
    volume: String,
}

//impl display for Row


async fn scrape( rows: & mut Vec<Row> ) -> Result<(), reqwest::Error> {
    let response = reqwest::get("https://www.boursier.com/indices/composition/cac-40-FR0003500008,FR.html").await?;
    let body = response.text().await?;
    let cursor = Cursor::new(body);
    let document = select::document::Document::from_read(cursor).unwrap();
    let table = document.find(select::predicate::Name("table")).next();
    if let Some(table) = table {

        for row in table.find(select::predicate::Name("tr")) {
            let cells: Vec<_> = row.find(select::predicate::Name("td"))
                .map(|cell| cell.text().trim().to_string())
                .collect();
            if cells.len() == 7 {
                let row_data = Row {
                    name: cells[0].clone(),
                    rate: cells[1].clone(),
                    variation: cells[2].clone(),
                    opening: cells[3].clone(),
                    high: cells[4].clone(),
                    low: cells[5].clone(),
                    volume: cells[6].clone(),
                };
                rows.push(row_data);
            }
          else {
            println!("Error: cells.len() = {}", cells.len()); }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let mut rows: Vec<Row> = Vec::new();
    loop {
        scrape(& mut rows).await?;
        println!("rows.len() = {}", rows.len());
        for row in &rows {
            println!("{} {} {} {} {} {} {}", row.name, row.rate, row.variation, row.opening, row.high, row.low, row.volume);
        }
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
    Ok(())
}


