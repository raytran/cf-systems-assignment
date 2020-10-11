#[cfg(test)]
mod requests {
    use cf_systems_assignment::RequestHandler;
    use cf_systems_assignment::status_code::StatusCode;

    #[test]
    fn simple_examplecom_request() {
        let response = RequestHandler::make_request_from_url("http://example.com".parse().unwrap()).unwrap();
        assert_eq!(response.status_code, StatusCode::Ok);
    }

    #[test]
    fn test_cf_links(){
        let url = "https://cf-general-assignment.raytran.workers.dev/links".to_string();
        let response = RequestHandler::make_request_from_url(url).unwrap();
        assert_eq!(response.status_code, StatusCode::Ok);
        assert!(response.data.contains("cloudflare"));
    }

    #[test]
    fn test_404_page(){
        let url = "http://web.mit.edu/notreal".parse().unwrap();
        let response = RequestHandler::make_request_from_url(url).unwrap();
        assert_eq!(response.status_code, StatusCode::NotFound);
    }

    #[test]
    fn test_profile_cf(){
        let url = "https://cf-general-assignment.raytran.workers.dev/links".to_string();
        let profile = RequestHandler::profile_from_url(url, 10).unwrap();
        println!("{}", profile);
    }
}