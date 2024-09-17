use std::fs::File;
use std::path::PathBuf;

use kuchiki::traits::*;
use kuchiki::parse_html;

use crate::json::Article;

pub struct HtmlProspect {
    pub html: String,
    pub r_articles: Vec<Article>,
    pub fr_articles: Vec<Article>,
    pub l_articles: Vec<Article>,
    pub fl_articles: Vec<Article>,
}

impl HtmlProspect {
    pub fn construct(file: &PathBuf) -> Result<Self, Box<dyn std::error::Error>> {
        let html = match std::fs::read_to_string(file) {
            Ok(html) => {
                html
            }
            Err(e) => {
                return Err(Box::new(e));
            }
        };

        let r_articles: Vec<Article> = Vec::new();
        let fr_articles: Vec<Article> = Vec::new();
        let l_articles: Vec<Article> = Vec::new();
        let fl_articles: Vec<Article> = Vec::new();

        Ok(Self {
            html,
            r_articles,
            fl_articles,
            l_articles,
            fr_articles
        })
    }
    pub fn populate(&mut self, articles: Vec<Article>) {
        todo!()
    }
}