![build](https://github.com/daite/ktorrent-rs/workflows/Rust/badge.svg)
[![Build Status](https://travis-ci.com/daite/ktorrent.svg?branch=main)](https://travis-ci.com/daite/ktorrent)
[![docs](https://docs.rs/ktorrent/badge.svg)](https://docs.rs/ktorrent)
# ktorrent
> Scraping korean torrent sites.
# example
```rust
use reqwest;
use reqwest::header::USER_AGENT;
use ktorrent::Document;
use ktorrent::find_child_attr_by_tag;

fn main() {
    let url = "https://torrentsir31.com/bbs/board.php?bo_table=movie&wr_id=15846";
    let client = reqwest::blocking::Client::new();
    let res = client.get(url)
                    .header(USER_AGENT, "Mozilla/5.0 (Macintosh; Intel Mac OS X 11_2_0)")
                    .send().unwrap();
    let doc = Document::from_read(res).unwrap();
    let result = find_child_attr_by_tag(
                  &doc, 
                  "list-group", 
                  "a", 
                  "href"
                );
    assert_eq!("magnet:?xt=urn:btih:dac87e714c3adf0fe073236ef32acafb6931ae63", result[1]);
}
```
## status
| No |      site     |             url             | status |
|:--:|:-------------:|:---------------------------:|:------:|
|  1 |    ttobogo    | https://ttobogo.net         |    O   |
|  2 |   torrentube  | https://torrentube.to       |     O  |
|  3 |     tshare    | https://tshare.org          |     O  |
|  4 | torrentmobile | https://torrentmobile15.com |    O   |
|  5 | torrentview   | https://torrentview28.com  |    O   |
|  6 | torrentsir   | https://torrentsir31.com |    O   |
|  7 | torrentj   | https://torrentj32.com |    O   |
|  8 | torrentqq   | https://torrentqq73.com |    O   |
|  9 | torrenttip   | https://torrenttip19.com |    O   |

