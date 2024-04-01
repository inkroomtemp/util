use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(),dyn std::error::Error>>{
    let resp = request::get("https://httpbin.org/ip")
        .await?
        .json::<HashMap<string,String>>()
        .await?;
    println!("{resp:#?}");
    Ok(())
}