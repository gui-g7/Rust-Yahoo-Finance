use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
enum FetchError {
    BadRequest,
    NoEnvironment,
    HTTPError,
}

fn fetch(url: &str, params: &HashMap<&str, &str>, devel: &str) -> Result<(), FetchError> {
    println!("Fetching URL: {}", url);
    println!("Params: {:?}", params);
    match devel {
        "search-noOpts.json" => return Err(FetchError::BadRequest),
        "pageWith404andJson.fake.json" => return Err(FetchError::HTTPError),
        "pageWithUnknownError.json" => return Err(FetchError::HTTPError),
        _ => return Ok(()),
    }
}

fn main() {
    let env: HashMap<&str, &str> = HashMap::new();
    let opts: HashMap<&str, &str> = HashMap::new();
    let yahoo_finance_fetch = |url: &str, params: &HashMap<&str, &str>, devel: &str| {
        fetch(url, params, devel);
    };
    let url = "https://query2.finance.yahoo.com/v1/finance/search";
    let params: HashMap<&str, &str> = HashMap::new();
    let devel = "search-noOpts.json";
}
