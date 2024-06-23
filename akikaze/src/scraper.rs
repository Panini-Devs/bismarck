use std::fs::{DirEntry, read_dir, File};
use std::io::BufReader;
use std::env;

use tokio::task;
use tracing::debug;

use crate::schema::BannerData;

pub fn scrape() {
    let path = env::var("AKIKAZE_RES").expect(
        "PATH not set. Set it with the `AKIKAZE_RES` environment variable.",
    );

    let dir = read_dir(path).unwrap();

    let futures = FuturesUnordered::new();

    for entry in dir {
        let future = task::spawn_blocking(move || {
            let path = entry.unwrap().path();
            debug!("Scraping {}", &path.display());

            let file = File::open(path).unwrap();
            let reader = BufReader::new(file);

            let data: BannerData = serde_json::from_reader(reader).unwrap();
        });

        futures.push(future);
    }

    let results: Vec<_> = futures.collect().await;

    debug!("Scraped {} banners", results.len());
}
