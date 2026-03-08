use std::{sync::Arc, time::Duration};

use anyhow::{anyhow, Context, Result};
use scraper::{Html, Selector};
use tokio::{
    select,
    sync::{mpsc, Mutex},
    task::JoinSet,
    time::sleep,
};
use tokio_util::sync::CancellationToken;

#[tokio::main]
async fn main() -> Result<()> {
    let seed = "https://www.racefans.net";
    let (jobs_tx, jobs_rx) = mpsc::channel::<String>(1000);
    let jobs_tx = Arc::new(Mutex::new(jobs_tx));
    let jobs_rx = Arc::new(Mutex::new(jobs_rx));
    let mut tasks = JoinSet::new();

    for _ in 0..5 {
        let jobs_tx = jobs_tx.clone();
        let jobs_rx = jobs_rx.clone();
        tasks.spawn(async move {
            while let Some(url) = jobs_rx.lock().await.recv().await {
                let links = links(&url).await.unwrap_or_default();

                for link in links {
                    println!("{}", link);
                    jobs_tx.lock().await.send(link).await.unwrap();
                }
            }
        });
    }

    jobs_tx.lock().await.send(seed.to_ascii_lowercase()).await?;

    while let Some(_) = tasks.join_next().await {}

    Ok(())
}

async fn links(url: &str) -> Result<Vec<String>> {
    let response = reqwest::get(url).await?;
    let html = Html::parse_document(&response.text().await?);
    let selector = Selector::parse("a").map_err(|e| anyhow!(e.to_string()))?;

    Ok(html
        .select(&selector)
        .filter_map(|e| e.attr("href").map(|e| e.to_ascii_lowercase()))
        .collect())
}
