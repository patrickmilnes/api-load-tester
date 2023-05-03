mod run_result;
mod runner;
mod runner_options;
mod runner_results;
mod command_line_options;

use crate::run_result::RunResults;
use crate::runner::run_reqs;
use crate::runner_options::RunnerOptions;
use crate::runner_results::print_results_pretty;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This will be taken from json file
    let delay: i32 = 100;
    let total_num_reqs: i32 = 5000;
    let concurrent_reqs: i32 = 10;

    // Run http load runner
    let opts = RunnerOptions::new(delay, total_num_reqs, concurrent_reqs);
    let url = "http://localhost:9000/load/hello";

    let res: RunResults = run_reqs(opts, url).await?;

    println!("{:?}", res.get(0).unwrap().request_results);
    // let _res = test().await?;
    // println!("done");

    // print_results_pretty(res);

    Ok(())
}

// delay -> int (ms)
// total number of requests -> int
// concurrent requests -> int

// some semantics
// A 'run' is a sending n requests, one after the other
// A 'req' is one http request
// A 'runner' is n concurrent runs