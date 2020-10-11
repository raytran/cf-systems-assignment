use std::cmp;
use std::io::{Error, Read, Write};
use std::net::TcpStream;
use std::time::Instant;

use crate::profile_result::ProfileResult;
use crate::request::Request;
use crate::response::Response;
use crate::status_code::StatusCode;

mod request;

// Public members
pub mod profile_result;
pub mod response;
pub mod status_code;

pub struct RequestHandler {}

impl RequestHandler {
    /// Makes a request from a given URL
    /// Returns Response if successful
    pub fn make_request_from_url(url: String) -> Result<Response, Error> {
        return RequestHandler::make_request(Request::new_get(url)?);
    }

    /// Makes a {request} for {times}
    /// Returns the time in milliseconds for the request to complete
    /// Requires times > 0, url is a full url
    pub fn profile_from_url(url: String, times: usize) -> Result<ProfileResult, Error> {
        return RequestHandler::profile(Request::new_get(url)?, times);
    }

    /// Creates a Response from a Request
    fn make_request(request: Request) -> Result<Response, Error> {
        let mut stream = TcpStream::connect(format!("{}:{}", request.get_host(), request.get_port()))?;
        stream.write(request.to_string().as_ref())?;
        let mut buffer = String::new();
        let byte_count = stream.read_to_string(&mut buffer)?;
        Ok(Response::from_raw(byte_count, buffer))
    }

    /// Profiles from a Request
    fn profile(request: Request, times: usize) -> Result<ProfileResult, Error> {
        assert!(times > 0);
        let url = request.get_url();
        let mut all_times = Vec::new();
        let mut all_sizes = Vec::new();
        let mut error_codes = Vec::new();
        let mut success_count = 0;
        for _i in 0..times {
            let before = Instant::now();
            let response = RequestHandler::make_request(request.clone())?;
            let ms_taken = before.elapsed().as_millis() as usize;
            if response.status_code == StatusCode::Ok {
                success_count += 1;
            } else {
                error_codes.push(response.status_code.clone());
            }
            all_sizes.push(response.byte_count);
            all_times.push(ms_taken);
        }
        // Sort all_times to easily find median
        all_times.sort();
        Ok(ProfileResult {
            url,
            num_requests: times,
            fastest_time_ms: *all_times.iter().min().unwrap(),
            slowest_time_ms: *all_times.iter().max().unwrap(),
            mean_time_ms: all_times.iter().sum::<usize>() as usize / all_times.len(),
            median_time_ms: *all_times.get(all_times.len() / 2).unwrap(),
            success_percentage: success_count as f64 / times as f64,
            smallest_response_bytes: *all_sizes.iter().min().unwrap(),
            largest_response_bytes: *all_sizes.iter().max().unwrap(),
            error_codes,
        })
    }
}

#[cfg(test)]
mod make_requests {
    use crate::request::Request;
    use crate::RequestHandler;
    use crate::status_code::StatusCode;
    use crate::status_code::StatusCode::RequestedRangeNotSatisfiable;

    #[test]
    fn simple_request() {
        let req = Request::new_get("https://cf-general-assignment.raytran.workers.dev/links".to_string());
        let response = RequestHandler::make_request(req.unwrap()).unwrap();
        assert_eq!(response.status_code, StatusCode::Ok)
    }

    #[test]
    fn test_404_page() {
        let req = Request::new_get("http://web.mit.edu/notreal".parse().unwrap()).unwrap();
        let response = RequestHandler::make_request(req).unwrap();
        assert_eq!(response.status_code, StatusCode::NotFound)
    }

    #[test]
    fn test_profile_cf() {
        let req = Request::new_get("https://cf-general-assignment.raytran.workers.dev/links".to_string()).unwrap();
        let profile = RequestHandler::profile(req, 10).unwrap();
        println!("{}", profile);
    }
}

