use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BookEntity {
    pub id: Option<i32>,
    pub title: String,
    pub author: String,
    pub pages: i32,
}
