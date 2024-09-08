use crate::books::domain::entity::book::BookEntity;

pub trait BookRepository {
    fn get_all(&mut self) -> Result<Vec<BookEntity>, String>;
    fn create(&mut self, _book: BookEntity) -> Result<BookEntity, String>;
    fn get_by_id(&self, _id: i32) -> Result<BookEntity, String>;
    fn update(&self, _book: &BookEntity) -> Result<BookEntity, String>;
    fn delete(&self, _id: i32) -> Result<(), String>;
}
