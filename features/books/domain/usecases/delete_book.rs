use crate::books::domain::repository::book_repository::BookRepository;

pub fn delete_book<R: BookRepository>(repository: &mut R, id: i32) -> Result<(), String> {
    repository.delete(id)
}
