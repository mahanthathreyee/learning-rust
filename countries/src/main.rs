use url::Url;
use serde_json::{Value};

const COUNTRIES_API: &str = "https://restcountries.eu/rest/v2/name";

struct RequestHandler {
    client: reqwest::Client
}

impl RequestHandler {
    pub fn new() -> RequestHandler {
        RequestHandler {
            client: reqwest::Client::builder().build().unwrap()
        }
    }

    fn get_url(&self, country_name: String) ->Url {
        let country_url = format!("{}/{}", COUNTRIES_API, country_name.as_str());
        let country_url = match Url::parse(country_url.as_str()) {
            Ok(t) => t,
            Err(_) => panic!("Error constructing url")
        };

        country_url
    }
    
    pub fn parse_json(&self, response: &str) ->Value {
        let v: Value = match serde_json::from_str(response) {
            Ok(t) => t,
            Err(_) => panic!("Error parsing json")
        };
        return v
    }

    pub async fn request_country_data(&self, country_name: String) ->Value {
        let country_url = self.get_url(country_name);
        println!("URL = {}", country_url.to_string());
        let response = match self.client.get(country_url).send().await {
            Ok(t) => t,
            Err(_) => panic!("Error contacting country API server")
        };

        if response.status().is_success() {
            println!("success!");
        }

        let response = match response.text().await {
            Ok(t) => t,
            Err(_) => panic!("Error parsing json")
        };

        let response = self.parse_json(response.as_str());
        return response
    }
}

#[tokio::main]
async fn main() {
    let request_handler = RequestHandler::new();
    let response = request_handler.request_country_data("USA".to_string()).await;
    println!("{:#?}", response);
}
