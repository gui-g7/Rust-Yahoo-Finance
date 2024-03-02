use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;
use serde::{Deserialize, Serialize};
#[derive(Debug, Deserialize, Serialize)]
struct FakeResponse {
    headers: HashMap<String, String>,
    body: String,
}

impl FakeResponse {
    fn new(headers: HashMap<String, String>, body: String) -> Self {
        Self { headers, body }
    }

    fn json(&self) -> Result<HashMap<String, String>, io::Error> {
        unimplemented!()
    }

    fn text(&self) -> &str {
        &self.body
    }
}
fn url_hash(url: &str) -> u64 {
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    hasher.finish()
}
fn fetch_devel(url: &str, fetch_options: HashMap<&str, &str>) -> Result<FakeResponse, io::Error> {
    let fetch_devel_env = std::env::var("FETCH_DEVEL").unwrap_or_else(|_| String::from(""));
    if fetch_devel_env == "nocache" {
        unimplemented!()
    } else {
        let url = url.replace("https://query1.finance.yahoo.com", "https://query2.finance.yahoo.com");
        let orig_url = url.clone();
        let url = url.replace(&[("?&"), ("crumb="), ("[^?&]+")].join(""), "");
        let filename = format!("../../tests/http/{}", url_hash(&url));
        if Path::new(&filename).exists() {
            let content = fs::read_to_string(&filename)?;
            let fake_response: FakeResponse = serde_json::from_str(&content)?;
            return Ok(fake_response);
        } else {
            unimplemented!()
        }
    }
}

fn main() {
    let url = "https://query1.finance.yahoo.com/v1/finance/search";
    let mut fetch_options = HashMap::new();
    fetch_options.insert("devel", "search-noOpts.json");

    match fetch_devel(url, fetch_options) {
        Ok(response) => {
            println!("Headers: {:?}", response.headers);
            println!("Body: {}", response.text());
        }
        Err(err) => println!("Error: {}", err),
    }
}
