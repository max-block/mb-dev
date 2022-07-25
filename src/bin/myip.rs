use reqwest::blocking::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Response {
    origin: String,
}

fn main() {
    let res = Client::new().get("https://httpbin.org/ip").send().unwrap();
    let res: Response = res.json().unwrap();
    println!("{}", res.origin);
}
