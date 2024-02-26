#![allow(unused_variables)]
use futures::{stream, StreamExt};
use lazy_static::lazy_static;
use reqwest::Client;
use std::{
    sync::{Arc, Mutex},
    time::Instant,
};

lazy_static! {
    static ref CLIENT: Client = Client::new();
}

const BASE_URL: &'static str = "https://flixhq.to";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let now = Instant::now();

    // This is used later in the code to get the page ids from
    // let page_html = CLIENT
    //     .get(format!(
    //         "{}/search/{}?page={}",
    //         BASE_URL, "example-query", 1
    //     ))
    //     .send()
    //     .await?
    //     .text()
    //     .await?;

    // Page ids from the page_html
    let ids = vec![
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
        Some(String::from("movie/watch-wonka-103654")),
    ];

    let mut urls = vec![];

    for id in ids.iter().flatten() {
        let url = format!("{}/{}", BASE_URL, id);
        urls.push(url);
    }

    let bodies = stream::iter(urls.clone())
        .enumerate()
        .map(|(index, url)| {
            let client = &CLIENT;
            async move {
                let resp = client.get(url).send().await?;
                resp.text().await.map(|text| (index, text))
            }
        })
        .buffer_unordered(urls.len());

    let results: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    bodies
        .for_each(|result| {
            let urls = urls.clone(); // Clone urls again for each closure
            let results = Arc::clone(&results);
            async move {
                match result {
                    Ok((index, text)) => {
                        let url = &urls[index];
                        let id = url.splitn(4, "/").collect::<Vec<&str>>()[3];
                        results.lock().unwrap().push(id.to_string());
                    }
                    Err(err) => {
                        eprintln!("Error processing url: {}", err);
                    }
                }
            }
        })
        .await;

    let results = Arc::try_unwrap(results).unwrap().into_inner().unwrap();

    println!("{:#?}", results);

    let elapsed = now.elapsed();
    println!("{:#?}", elapsed);

    Ok(())
}
