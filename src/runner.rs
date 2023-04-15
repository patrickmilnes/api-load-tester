use std::arch::asm;
use std::collections::HashMap;
use std::error::Error;
use nanoid::nanoid;
use reqwest::Response;
use crate::runner_options::{RunnerOptions, RunOptions};
use crate::run_result::RunResults;

pub async fn test() -> Result<(), Box<dyn Error>> {
    let url = "http://localhost:5156/load/hello";
    let resp: Response = reqwest::get(url).await?;
    let body = json::<HashMap<String, String>>().await?;
    println!("{:?}", resp);

    Ok(())
}


pub async fn run_reqs(options: RunnerOptions, url: &str) -> Result<(), Box<dyn Error>> {
    // Spawn tokio threads based on `concurrent_reqs`
    // Each threat given thread id: 1, 2, 3, etc...
    // Put req results into some shared vector
    // Write results to file


    Ok(())
}

async fn run_requests(options: RunOptions) -> Result<(), Box<dyn Error>> {
    let id: String = nanoid!(6);
    let results: RunResults = Vec::new();
    for run in 1..options.num_reqs {
        let resp = reqwest::get(options.url.as_str())
            .await?
            .json::<HashMap<String, String>>()
            .await?;
    }
    Ok(())
}