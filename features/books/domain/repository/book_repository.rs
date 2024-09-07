use crate::books::domain::entity::book::BookEntity;

pub trait BookRepository {
    async fn create(&mut self, _book: &BookEntity) -> Result<BookEntity, String>;
    fn read(&self, _id: i32) -> Result<BookEntity, String>;
    fn update(&self, _book: &BookEntity) -> Result<BookEntity, String>;
    fn delete(&self, _id: i32) -> Result<BookEntity, String>;
}