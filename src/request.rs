use std::fmt;
use std::io::{Error, ErrorKind};

use url::Url;

/// Represents an HTTP request; only supports GET (for now!)
#[derive(Clone)]
pub struct Request {
    url: String,
    port: usize,
    host: String,
    request_line: String,
    headers: String,
    body: String,
}

impl Request {
    /// Create a new GET request
    pub fn new_get(url: String) -> Result<Request, Error> {
        if let Ok(parsed_url) = Url::parse(url.as_str()) {
            return Ok(Request {
                url,
                port: 80,
                host: parsed_url.host_str().unwrap_or("").parse().unwrap(),
                request_line: format!("GET {} HTTP/1.1", parsed_url.path()),
                headers: format!("Host: {} \r\n{}", parsed_url.host_str().unwrap_or(""), "Connection: close"),
                body: String::from("\r\n"),
            });
        }
        Err(std::io::Error::new(ErrorKind::InvalidInput, "Error parsing URL"))
    }

    /// Returns the host for this request
    pub fn get_host(&self) -> String {
        self.host.clone()
    }

    /// Return the target url for this request
    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    /// Return the target port for this request (80 for HTTP)
    pub fn get_port(&self) -> usize {
        return self.port;
    }

    /// Convert this Request into a String
    pub fn to_string(&self) -> String {
        let mut request = String::new();
        request.push_str(&self.request_line.as_str());
        request.push_str("\r\n");
        request.push_str(&self.headers.as_str());
        request.push_str("\r\n");
        request.push_str(&self.body.as_str());
        return request;
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

#[cfg(test)]
mod format_requests {
    use crate::request::Request;

    #[test]
    fn simple_request() {
        let req = Request::new_get("https://cf-general-assignment.raytran.workers.dev/".to_string());
        assert!(req.is_ok());
    }

    #[test]
    fn test_broken_link() {
        let req = Request::new_get("http//completenonsense.com/".to_string());
        assert!(req.is_err())
    }
}