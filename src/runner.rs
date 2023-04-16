use crate::run_result::{RequestResult, RequestResults, RunResult};
use crate::runner_options::{RunOptions, RunnerOptions};
use nanoid::nanoid;
use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;

pub async fn run_reqs(options: RunnerOptions, url: &str) -> Result<(), Box<dyn Error>> {
    // Spawn tokio threads based on `concurrent_reqs`
    // Each threat given thread id: 1, 2, 3, etc...
    // Put req results into some shared vector
    // Write results to file

    todo!("See comments at top of function")
}

async fn run(options: RunOptions) -> Result<RunResult, Box<dyn Error>> {
    let id: String = nanoid!(6);
    let mut results: RequestResults = Vec::new();
    for run in 1..options.num_reqs {
        // Get system time to time the request
        let now = SystemTime::now();

        // Make http request
        let res = reqwest::get(options.url.as_str()).await?;

        let time_taken = now.elapsed()?.as_millis();
        let req_result = RequestResult {
            run_index: run,
            status_code: res.status().as_u16(),
            body: res.json::<HashMap<String, String>>().await?,
            time_taken,
        };

        results.push(req_result);
    }

    Ok(RunResult::new(id, results))
}
