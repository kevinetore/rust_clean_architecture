use crate::books::domain::entity::book::BookEntity;
use crate::books::domain::repository::book_repository::BookRepository;

pub fn get_books<R: BookRepository>(repository: &mut R) -> Result<Vec<BookEntity>, String> {
    repository.get_all()
}
