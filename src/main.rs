use clap::App;
use clap::Arg;

use cf_systems_assignment::RequestHandler;
use cf_systems_assignment::status_code::StatusCode::{RequestedRangeNotSatisfiable, RequestHeaderFieldsTooLarge};

fn main() {
    fn is_number(v: String) -> Result<(), String> {
        if usize::from_str_radix(&v, 10).is_ok() { return Ok(()); }
        Err(String::from("Not a number."))
    }
    let matches = App::new("CF 2020 Systems Engineering Assignment")
        .version("1.0")
        .author("Raymond Tran <raytran@mit.edu>")
        .about("Measure how fast your website is over HTTP!")
        .arg(Arg::with_name("url")
            .required(true)
            .help("HTTP Website to visit/profile, must be in the form http://<host>.<TLD>/<subdirectories>. If 'http://' is absent, it is prepended to the argument.,")
            .takes_value(true))
        .arg(Arg::with_name("profile")
            .help("Number of times to make GET requests. Must be a positive integer. If omitted, prints to console result of GET request.")
            .validator(is_number)
            .short("p")
            .long("profile")
            .takes_value(true)
        )
        .get_matches();

    if let Some(o) = matches.value_of("url") {
        let mut url = String::from(o);
        // Can't do https yet...
        url = url.replace("https://", "http://");
        if url.len() < 7 || &url[0..7] != "http://" {
            url = format!("http://{}", url);
        }
        println!("{}", url);
        if let Some(n) = matches.value_of("profile") {
            // Profile with num_requests
            let num_requests = usize::from_str_radix(n, 10).unwrap();
            let res = RequestHandler::profile_from_url(url, num_requests);
            match res {
                Ok(profile) => {
                    println!("{}", profile);
                }
                Err(e) => {
                    println!("Request error.");
                    print!("{:?}", e);
                }
            }
        } else {
            // Not profiling, just print GET response
            match RequestHandler::make_request_from_url(url) {
                Ok(response) => {
                    println!("{}", response);
                }
                Err(e) => {
                    println!("Request error.");
                    print!("{:?}", e);
                }
            }
        }
    }
}
