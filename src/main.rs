extern crate reqwest;

#[tokio::main]
async fn main() {
    match reqwest::get("http://youtube.com").await {
        Ok(response) => {
            println!("Status: {}", response.status());
            println!("Headers:\n{:#?}", response.headers());
        }
        Err(e) => println!("Error: {}", e),
    }
}
