const HTML: &str = include_str!("../src/page.html");

use axum::{
    handler,
    response::Html,
    routing::{get, post},
    Router,
};

use std::fs::File;
use std::io::*;


use axum::{extract::Path, Form, Json};
use serde::{Deserialize, Serialize};

pub async fn get_handler() -> Html<String> {
    Html(HTML.to_owned())
}

pub async fn post_handler() -> Html<String> {
    Html("why tf u posting here".to_owned())
}