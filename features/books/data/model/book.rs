use serde::{Deserialize, Serialize};
use crate::books::domain::entity::book::BookEntity;
use std::convert::{From, Into};
use diesel::{Insertable, Queryable, Selectable};
use core_diesel::tables::schema::books;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Insertable)]
#[diesel(table_name = books)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub pages: i32,
}

impl From<&BookEntity> for BookModel {
    fn from(entity: &BookEntity) -> Self {
        BookModel {
            id: entity.id,
            title: entity.title.clone(),
            author: entity.author.clone(),
            pages: entity.pages,
        }
    }
}

impl Into<BookEntity> for &BookModel {
    fn into(self) -> BookEntity {
        BookEntity {
            id: self.id,
            title: self.title.clone(),
            author: self.author.clone(),
            pages: self.pages,
        }
    }
}