![build](https://github.com/daite/ktorrent-rs/workflows/Rust/badge.svg)
[![Build Status](https://travis-ci.com/daite/ktorrent.svg?branch=main)](https://travis-ci.com/daite/ktorrent)
![crates](https://img.shields.io/badge/crates.io-krorrent%20=%20%220.1.0%22-brightgreen.svg)
# ktorrent
> collect test data for scraping korean torrent sites.
## Status
| No |      site     |             url             | status |
|:--:|:-------------:|:---------------------------:|:------:|
|  1 |    ttobogo    | https://ttobogo.net         |    O   |
|  2 |   torrentube  | https://torrentube.to       |     O  |
|  3 |     tshare    | https://tshare.org          |     O  |
|  4 | torrentmobile | https://torrentmobile15.com |    O   |
|  5 | torrentview   | https://torrentview28.com  |    O   |
|  6 | torrentsir   | https://torrentsir31.com |    O   |
|  7 | torrentj   | https://torrentj32.com |    O   |
## Example
```rust
use example::*;
use std::sync::{Arc, Mutex};
use std::error::Error;

const HOST_URL: &'static str = "https://torrentsir31.com/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    run().await?;
    Ok(())
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let search_words = "동상이몽2";
    let data = get_data(search_words);
    let mut tasks = vec![];
    let r: Vec<(String, String)> = vec![];
    let result = Arc::new(Mutex::new(r));
    for d in data.await? {
        let result = Arc::clone(&result);
        tasks.push(tokio::spawn(async move {
            let bbs_doc = get_doc(&d.1).await.unwrap();
            let magnet = get_data_by_class_name(
                &bbs_doc, "list-group", "a", "href")[1];
           let mut r = result.lock().unwrap();
           r.push((d.0, magnet.to_string()));
        }));
    }
    for task in tasks {
        task.await?;
    }
    let p = &mut *result.lock().unwrap();
    p.sort();
    p.reverse();
    print_table(p.to_vec());
    Ok(())
}

pub async fn get_data(search_words: &str) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let search_url = format!("{}/bbs/search.php?search.php&stx={}", HOST_URL, search_words);
    let base_url = format!("{}/bbs", HOST_URL);
    let search_doc = get_doc(&search_url).await?;
    let titles = get_data_by_tag_name(&search_doc, "b", "sch_word");
    let urls = get_data_by_class_name(&search_doc, "media-heading", "a", "href");
    let k: Vec<String> =  urls.iter().map(|x| format!("{}{}", base_url, x.trim_start_matches(|c| c == '.'))).collect();
    let data: Vec<(String, String)> = titles.into_iter().zip(k.into_iter()).collect();
    Ok(data)
}
```
## Output
![image](https://raw.githubusercontent.com/daite/ktorrent-rs/main/images/sample.png)
