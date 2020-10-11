use std::fmt;

use regex::Regex;

use crate::status_code::StatusCode;

/// Represents an response from an HTTP Request
pub struct Response {
    /// Status code of this response
    pub status_code: StatusCode,
    /// Size of the response in bytes
    pub byte_count: usize,
    /// Raw data of this response
    pub data: String,
}

impl Response {
    /// Returns a new Response, from a raw_data string with size byte_count
    pub(crate) fn from_raw(byte_count: usize, raw_data: String) -> Response {
        let mut status_code = 0;
        let re = Regex::new(r"([0-9][0-9][0-9])").unwrap();
        'find_status: for line in raw_data.lines() {
            for word in line.split_whitespace() {
                if re.is_match(word) {
                    status_code = usize::from_str_radix(word, 10).unwrap();
                    break 'find_status;
                }
            }
        }
        Response {
            status_code: StatusCode::from_num(status_code),
            byte_count,
            data: raw_data,
        }
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.status_code, self.data)
    }
}

#[cfg(test)]
mod make_responses {
    use crate::response::Response;
    use crate::status_code::StatusCode;

    #[test]
    fn simple_request() {
        let raw = "HTTP/1.1 200 OK\r\n\
                         Date: Sat, 10 Oct 2020 20:42:41 GMT \r\n\
                         Content-Type: application/json;charset=UTF-8\r\n\
                         Content-Length: 387\r\n\
                         Connection: close\r\n\

                         [";

        let res = Response::from_raw(400, raw.parse().unwrap());
        assert_eq!(res.status_code, StatusCode::Ok);
    }
}

