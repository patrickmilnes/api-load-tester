use std::collections::HashMap;

#[derive(Debug)]
pub struct RequestResult {
    pub run_index: i32,
    pub status_code: u16,
    pub body: HashMap<String, String>,
    pub time_taken: u128,
}

pub type RequestResults = Vec<RequestResult>;

#[derive(Debug)]
pub struct RunResult {
    pub id: String,
    pub request_results: RequestResults
}

pub type RunResults = Vec<RunResult>;

impl RunResult {
    pub fn new(id: String, request_results: RequestResults) -> Self {
        RunResult {
            id,
            request_results,
        }
    }
}
