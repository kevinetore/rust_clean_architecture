use crate::books::domain::entity::book::BookEntity;

pub trait BookRepository {
    fn create(&mut self, _book: BookEntity) -> Result<BookEntity, String>;
    fn get(&self, _id: i32) -> Result<BookEntity, String>;
    fn update(&self, _book: &BookEntity) -> Result<BookEntity, String>;
    fn delete(&self, _id: i32) -> Result<(), String>;
}