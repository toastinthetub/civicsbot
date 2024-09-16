// const HTML: &str = include_str!("../src/index.html");

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
    let mut file = File::open("./src/index.html").unwrap();
    let mut html = String::new();   
    file.read_to_string(&mut html).unwrap();
    Html(html)
}

pub async fn post_handler() -> Html<String> {
    Html("why tf u posting here".to_owned())
}