mod html_edit;
mod handlers;
mod json;

use std::sync::{Arc, Mutex};

use axum::{
    handler,
    response::Html,
    routing::{get, post},
    Router,
};

use axum::{extract::Path, Form, Json};
use json::ApiConfiguration;
use serde::{Deserialize, Serialize};

use crate::handlers::*;

#[tokio::main]
async fn main() {

    let articles: Arc<Mutex<Vec<crate::json::Article>>> = Arc::new(Mutex::new(Vec::new()));

    let server_thread = tokio::spawn(async {
        let app: Router = Router::new().route("/", get(get_handler).post(post_handler));

        let listener = match tokio::net::TcpListener::bind("0.0.0.0:42069").await {
            Ok(listener) => {
                listener
            }
            Err(e) => {
                eprintln!("Failed to bind a TCP listener! Shitting pants and giving up");
                std::process::exit(1);
            }
        };

        axum::serve(listener, app).await.unwrap();
    });

    let news_thread = tokio::spawn(async { // ok dope this works
        let api_config = ApiConfiguration::construct(16);

        let articles = api_config.get_articles().await.unwrap();
        println!("Here are the titles of the fetched articles:");
        for (i, article) in articles.iter().enumerate() {
            println!("Article {}: {}", i + 1, article.title);
        }
        loop {
            println!("This is a thread.");
            tokio::time::sleep(std::time::Duration::from_secs(3)).await;

        }
    });

    let _ = tokio::join!(server_thread, news_thread);
}
