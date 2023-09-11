use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogPostSummary {
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub image: Option<String>,
}

impl BlogPostSummary {
    pub fn new(title: String, summary: String, tags: Vec<String>) -> Self {
        Self {
            title,
            summary,
            tags,
            image: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlogPost {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub image: Option<String>,
}
