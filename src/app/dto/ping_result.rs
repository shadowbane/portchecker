use std::time::Duration;

use crate::app::traits::check_result::CheckResult;
use crate::app::traits::failed_result::HasFailedResults;
use crate::app::traits::successful_result::HasSuccessfulResults;

#[derive(Debug)]
pub struct PingResult {
    pub host: String,
    pub domain: String,
    pub port: u16,
    pub sequence: u8,
    pub ttl: Duration,
    pub success: bool,
}

#[derive(Debug)]
pub struct PingResults {
    pub results: Vec<PingResult>,
}

pub trait SuccessField {
    fn success(&self) -> bool;
}

// Implementing SuccessField for PingResult
impl SuccessField for PingResult {
    fn success(&self) -> bool {
        self.success
    }
}

impl HasFailedResults<PingResult> for PingResults {
    fn results(&self) -> &Vec<PingResult> {
        return &self.results;
    }
}

impl HasSuccessfulResults<PingResult> for PingResults {
    fn results(&self) -> &Vec<PingResult> {
        return &self.results;
    }
}

impl CheckResult<PingResult> for PingResults {
    fn average(&self) -> f64 {
        let successful_results = self.successful_results();

        if successful_results.is_empty() {
            return 0.0; // or handle this case as needed
        }

        let total_ttl: u64 = successful_results.iter().map(|r| r.ttl.as_millis() as u64).sum();
        let average_ttl = total_ttl as f64 / successful_results.len() as f64;

        // Round to three digits
        (average_ttl * 1000.0f64).round() / 1000.0
    }

    fn min(&self) -> f64 {
        let successful_results = &self.successful_results();

        if successful_results.is_empty() {
            return 0.0;
        }

        let min_ttl = successful_results.iter().map(|r| r.ttl.as_micros());
        let min: f64 = min_ttl.min().unwrap_or(0) as f64;

        // Convert to seconds and round to three digits
        min / 1000.0f64
    }

    fn max(&self) -> f64 {
        let successful_results = &self.successful_results();

        if successful_results.is_empty() {
            return 0.0;
        }

        let max_ttl = successful_results.iter().map(|r| r.ttl.as_micros());
        let max: f64 = max_ttl.max().unwrap_or(0) as f64;

        // Convert to seconds and round to three digits
        max / 1000.0f64
    }

    fn failed(&self) -> u8 {
        let failed_results = self.failed_results();

        return failed_results.len() as u8;
    }

    fn received(&self) -> u8 {
        let successful_results = self.successful_results();

        return successful_results.len() as u8;
    }

    fn transmitted(&self) -> u8 {
        return self.results.len() as u8;
    }
}
