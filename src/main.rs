mod run_result;
mod runner;
mod runner_options;

use crate::runner::run_reqs;
use crate::runner_options::RunnerOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This will be taken from json file
    let delay: i32 = 0;
    let total_num_reqs: i32 = 5000;
    let concurrent_reqs: i32 = 4;

    // Run http load runner
    let opts = RunnerOptions::new(delay, total_num_reqs, concurrent_reqs);
    let url = "http://localhost:5156/load/hello";

    let _res = run_reqs(opts, url).await?;

    // let _res = test().await?;
    // println!("done");

    Ok(())
}

// delay -> int (ms)
// total number of requests -> int
// concurrent requests -> int
