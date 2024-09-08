use serde::{Deserialize, Serialize};
use crate::books::domain::entity::book::BookEntity;
use std::convert::{From, Into};
use diesel::{Insertable, Queryable, Selectable};
use core_diesel::tables::schema::books;

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = books)]
pub struct BookModel {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub pages: i32,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = books)]
pub struct NewBookModel {
    pub title: String,
    pub author: String,
    pub pages: i32,
}

impl From<BookEntity> for NewBookModel {
    fn from(entity: BookEntity) -> Self {
        NewBookModel {
            title: entity.title,
            author: entity.author,
            pages: entity.pages,
        }
    }
}

impl Into<BookEntity> for BookModel {
    fn into(self) -> BookEntity {
        BookEntity {
            id: Some(self.id),
            title: self.title,
            author: self.author,
            pages: self.pages,
        }
    }
}