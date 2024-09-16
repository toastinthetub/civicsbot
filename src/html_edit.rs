use kuchiki::traits::*;
use kuchiki::parse_html;

use crate::json::Article;

pub struct HtmlProspect {
    pub r_articles: Vec<Article>,
    pub fr_articles: Vec<Article>,
    pub l_articles: Vec<Article>,
    pub fl_articles: Vec<Article>,
}

impl HtmlProspect {
    
}