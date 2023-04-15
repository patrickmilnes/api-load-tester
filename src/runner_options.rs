pub struct RunnerOptions {
    pub delay: i32,
    pub total_num_reqs: i32,
    pub concurrent_reqs: i32
}

// Options for sequential run
pub struct RunOptions {
    pub url: String,
    pub delay: i32,
    pub num_reqs: i32
}

impl RunnerOptions {
    pub fn new(delay: i32, total_num_reqs: i32, concurrent_reqs: i32) -> Self {
        RunnerOptions {
            delay,
            total_num_reqs,
            concurrent_reqs
        }
    }
}

impl RunOptions {
    pub fn new(url: String, delay: i32, num_reqs: i32) -> Self {
        RunOptions {
            url,
            delay,
            num_reqs
        }
    }
}
