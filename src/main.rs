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
use serde::{Deserialize, Serialize};

use crate::handlers::*;

#[tokio::main]
async fn main() {

    let articles: Arc<Mutex<Vec<crate::json::Article>>> = Arc::new(Mutex::new(vec![crate::json::Article::new_empty()]));

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

    let news_thread = tokio::spawn(async {
        loop {
            4
        }
    });

    let _ = tokio::join!(server_thread, news_thread);
}

async fn newsloop() {
    
}