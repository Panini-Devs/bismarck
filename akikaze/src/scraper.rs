use std::fs::{DirEntry, read_dir, File};
use std::io::BufReader;
use std::env;

use futures::stream::FuturesUnordered;
use tokio::task;
use tracing::debug;

use crate::schema::BannerData;

pub async fn scrape() {
    let path = env::var("AKIKAZE_RES").expect(
        "PATH not set. Set it with the `AKIKAZE_RES` environment variable.",
    );

    let dir = read_dir(path).unwrap();

    let futures = FuturesUnordered::new();

    let _ = dir.map(|entry| {
        let future = task::spawn(async move {
            get_banner_data(entry).await
        });

        futures.push(future);
    });

    // TODO: Get results from futures
}

async fn get_banner_data(entry: Result<DirEntry, std::io::Error>) -> BannerData {
    let path = entry.unwrap().path();
    debug!("Scraping {}", &path.display());

    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let data: BannerData = serde_json::from_reader(reader).unwrap();

    debug!("Banner data: {:?}", data);

    data
}