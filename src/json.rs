use serde::{Deserialize, Serialize};

const URL: &str = "https://newsapi.org/v2/top-headlines";
const API_KEY: &str = "9a59ccce07b843869847fddf6de18c5d"; // REMEMBER TO DELETE THIS BEFORE COMMIT // oops.


// https://newsapi.org/v2/top-headlines?apiKey=YOUR_API_KEY&country=us&pageSize=5

pub enum Country {
    US,
    UK
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleSource {
    id: Option<String>,  // id can be None
    name: String,
}

impl ArticleSource {
    pub fn new_empty() -> Self {
        let id: Option<String> = Some(String::new());
        let name = String::from("PLACEHOLDER");
        Self {
            id,
            name
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {

    source: ArticleSource,
    author: Option<String>,  // author can be None
    title: String,
    description: String,
    url: String,
    url_to_image: String,
    published_at: String,  // Consider using chrono for a more sophisticated date/time handling
    content: Option<String>,  // content can be None
}

impl Article {
    pub fn new_empty() -> Self { // returns an empty article, so that we can put something in the vector of Mutex
        let source = ArticleSource::new_empty();
        let author: Option<String> = Some(String::from("PLACEHOLDER AUTHOR"));
        let title = String::from("PLACEHOLDER TITLE");
        let description = String::from("PLACEHOLDER DESCRIPTION");
        let url = String::from("www.PLACEHOLDER.com");
        let url_to_image = String::from("www.PLACEHOLDER.com");
        let published_at = String::from("PL:AC:EH");
        let content = Some(String::from("PLACEHOLDER"));
        Self {
            source,
            author,
            title,
            description,
            url,
            url_to_image,
            published_at,
            content
        }
    }
}

pub struct ApiConfiguration {
    url: String,
    key: String,
    country: /*Country*/ String ,
    narticles: i32
}

impl ApiConfiguration {
    pub fn construct(n: i32) -> Self { // constructs ApiConfig instance. needs URL, key, and n
        let url = URL.to_owned();
        let key = API_KEY.to_owned();
        let country = "US".to_owned();
        let narticles = n;

        Self {
            url,
            key,
            country,
            narticles   
        }
    }
    
    pub async fn get_news_json(&self) -> String {
        // for now we will fetch 5 articles ...
        let mut endpoint: String = String::new();
        endpoint = format!("{}?apikey={}&country={}&pageSize={}", self.url, self.key, self.country, self.narticles);
        println!("constructed endpoint: '{}'", endpoint);
        return endpoint
    }
}