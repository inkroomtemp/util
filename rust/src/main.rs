use std::io::Write;
use std::process::exit;
use select::document::Document;
use select::node::Node;
use select::predicate::{Attr, Class, Name, Predicate};
fn main() {

    let r = reqwest::blocking::get("https://blog.rust-lang.org/").unwrap();

    let text = r.text().unwrap();

    let dom = Document::from(text.as_str());

    let x:Vec<_> = dom.find(Name("tr")).collect();

    for n in x {
        match n
            .find(Name("a"))
            .next() {
            None => {}
            Some(e) => {
                let title = e.text();
                if title.starts_with("Announcing Rust ") {
                    print!("{}",&title[16..]);
                    // std::io::stdout().flush().unwrap();
                    exit(0);
                }
            }
        }

    }

    print!("null");
    // std::io::stdout().flush().unwrap();

}
