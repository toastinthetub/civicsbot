use serde::{Deserialize, Serialize};
use reqwest;
use std::error::Error;

const URL: &str = "https://newsapi.org/v2/top-headlines";
const API_KEY: &str = "9a59ccce07b843869847fddf6de18c5d"; // REMEMBER TO DELETE THIS BEFORE COMMIT

pub enum Country {
    US,
    UK,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ArticleSource {
    pub id: Option<String>,  // id can be None
    pub name: String,
}

impl ArticleSource {
    pub fn new_empty() -> Self {
        let id: Option<String> = Some(String::new());
        let name = String::from("PLACEHOLDER");
        Self { id, name }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Article {
    pub source: ArticleSource,
    pub author: Option<String>,  // author can be None
    pub title: String,
    pub description: Option<String>, // description can be None
    pub url: String,
    pub url_to_image: Option<String>, // can be None
    pub published_at: String,  // Consider using chrono for better date/time handling
    pub content: Option<String>,  // content can be None
}

impl Article {
    pub fn new_empty() -> Self {
        let source = ArticleSource::new_empty();
        let author: Option<String> = Some(String::from("PLACEHOLDER AUTHOR"));
        let title = String::from("PLACEHOLDER TITLE");
        let description = Some(String::from("PLACEHOLDER DESCRIPTION"));
        let url = String::from("www.PLACEHOLDER.com");
        let url_to_image = Some(String::from("www.PLACEHOLDER.com"));
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
            content,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse {
    status: String,
    total_results: Option<i32>,
    articles: Vec<Article>,
}

pub struct ApiConfiguration {
    url: String,
    key: String,
    country: String, // Represent the country as a string
    narticles: i32,
}

impl ApiConfiguration {
    pub fn construct(n: i32) -> Self {
        let url = URL.to_owned();
        let key = API_KEY.to_owned();
        let country = "us".to_owned(); // Default to "us", can change later if needed
        let narticles = n;

        Self {
            url,
            key,
            country,
            narticles,
        }
    }

    pub async fn get_news_json(&self) -> Result<String, Box<dyn Error>> {
        // Construct the API endpoint URL
        let endpoint = format!(
            "{}?apiKey={}&country={}&pageSize={}",
            self.url, self.key, self.country, self.narticles
        );
        println!("constructed endpoint: '{}'", endpoint);

        // Send the request and retrieve the response as a string
        let response = reqwest::get(&endpoint).await?.text().await?;
        Ok(response)
    }

    // New method to get and deserialize articles into a Vec<Article>
    pub async fn get_articles(&self) -> Result<Vec<Article>, Box<dyn Error>> {
        // Fetch the news JSON string
        let json_response = self.get_news_json().await?;

        // Deserialize the JSON response into an ApiResponse struct
        let api_response: ApiResponse = serde_json::from_str(&json_response)?;

        // Return the articles from the ApiResponse
        Ok(api_response.articles)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Create the API configuration with 16 articles to retrieve
    let api_config = ApiConfiguration::construct(16);

    // Get the articles
    let articles = api_config.get_articles().await?;

    // Print the articles
    for (i, article) in articles.iter().enumerate() {
        println!("Article {}: {}", i + 1, article.title);
    }

    Ok(())
}
