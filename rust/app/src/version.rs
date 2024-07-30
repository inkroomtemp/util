use std::collections::HashMap;

use select::{document::Document, predicate::Name};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://blog.rust-lang.org/")
        .await?
        .text()
        .await?;

    let dom = Document::from(resp.as_str());
    let x: Vec<_> = dom.find(Name("tr")).collect();

    for n in x {
        match n.find(Name("a")).next() {
            None => {}
            Some(e) => {
                let title = e.text();
                if title.starts_with("Announcing Rust ") {
                    print!("version {}", &title[16..]);
                    // std::io::stdout().flush().unwrap();
                    std::process::exit(0);
                }
            }
        }
    }
    print!("null");
    Ok(())
}
