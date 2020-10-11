use std::fmt;

use crate::status_code::StatusCode;

/// The result of a profile test
pub struct ProfileResult {
    pub url: String,
    pub num_requests: usize,
    pub fastest_time_ms: usize,
    pub slowest_time_ms: usize,
    pub mean_time_ms: usize,
    pub median_time_ms: usize,
    pub success_percentage: f64,
    pub smallest_response_bytes: usize,
    pub largest_response_bytes: usize,
    pub error_codes: Vec<StatusCode>,
}

impl fmt::Display for ProfileResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut error_codes = String::new();
        for error in &self.error_codes {
            error_codes.push_str(&format!("{} ", *error as usize));
        }

        let mut url_header = String::new();
        for _i in 0..self.url.len() {
            url_header.push('-')
        }
        url_header.push('\n');
        url_header.push_str(self.url.as_str());
        url_header.push('\n');
        for _i in 0..self.url.len() {
            url_header.push('-')
        }

        write!(f, "{}\n\
                   Number of Requests.........{}\n\
                   Fastest Time (ms)..........{}\n\
                   Slowest Time (ms)..........{}\n\
                   Mean Time (ms).............{}\n\
                   Median Time (ms)...........{}\n\
                   Success Percentage.........{}\n\
                   Smallest Response (bytes)..{}\n\
                   Largest Response (bytes)...{}\n\
                   Error Codes:\n    {}",
               url_header,
               self.num_requests,
               self.fastest_time_ms,
               self.slowest_time_ms,
               self.mean_time_ms,
               self.median_time_ms,
               self.success_percentage,
               self.smallest_response_bytes,
               self.largest_response_bytes,
               error_codes)
    }
}

#[cfg(test)]
mod format_profile_results {
    use crate::profile_result::ProfileResult;
    use crate::status_code::StatusCode;

    #[test]
    fn print_profile() {
        println!("{}", ProfileResult {
            url: "fakewebsite.com".to_string(),
            num_requests: 0,
            fastest_time_ms: 0,
            slowest_time_ms: 0,
            mean_time_ms: 0,
            median_time_ms: 0,
            success_percentage: 0.0,
            error_codes: vec![StatusCode::NetworkConnectTimeoutError, StatusCode::NotFound],
            smallest_response_bytes: 0,
            largest_response_bytes: 0,
        })
    }
}

