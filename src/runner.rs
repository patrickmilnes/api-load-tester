use crate::run_result::{RequestResult, RequestResults, RunResult, RunResults};
use crate::runner_options::{RunOptions, RunnerOptions};
use nanoid::nanoid;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, SystemTime};
use tokio::task::JoinHandle;
use tokio::time::sleep;

pub async fn run_reqs(options: RunnerOptions, url: &'static str) -> Result<RunResults, Box<dyn Error>> {
    // Spawn tokio threads based on `concurrent_reqs`
    // Each threat given thread id: 1, 2, 3, etc...
    // Put req results into some shared vector
    // Write results to file
    println!("Running reqs");
    let reqs_per_run = options.total_num_reqs / options.concurrent_reqs;

    let mut handlers: Vec<JoinHandle<RunResult>> = Vec::new();
    let mut run_results: RunResults = Vec::new();

    let now = SystemTime::now();
    for n in 0..options.concurrent_reqs {
        println!("run: {}", n);
        let handle = tokio::spawn(async move {
            let opts = RunOptions::new(url.to_string(), options.delay, reqs_per_run);
            let res = run(opts).await;
            match res {
                Ok(r) => r,
                Err(e) => RunResult::new("error".to_string(), Vec::new())
            }
        });

        handlers.push(handle);
    }

    // Await for all runs to do their thing
    for handle in handlers {
        let out = handle.await.unwrap();
        run_results.push(out);
    }

    let time_taken = now.elapsed()?.as_secs();
    println!("Total time: {}", time_taken);

    Ok(run_results)
}

async fn run(options: RunOptions) -> Result<RunResult, Box<dyn Error>> {
    let id: String = nanoid!(6);
    let mut results: RequestResults = Vec::new();
    for req in 0..options.num_reqs {
        // Get system time to time the request
        let now = SystemTime::now();

        // Make http request
        let res = reqwest::get(options.url.as_str()).await?;

        // Record time taken
        let time_taken = now.elapsed()?.as_millis();

        // Build request struct
        let req_result = RequestResult {
            run_index: req,
            status_code: res.status().as_u16(),
            body: res.json::<HashMap<String, String>>().await?,
            time_taken,
        };

        // Add results to results vector
        results.push(req_result);
        sleep(Duration::from_millis(options.delay as u64)).await;
    }

    Ok(RunResult::new(id, results))
}
