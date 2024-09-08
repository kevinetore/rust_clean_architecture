use crate::books::domain::entity::book::BookEntity;
use crate::books::domain::repository::book_repository::BookRepository;

pub fn get_book<R: BookRepository>(
    repository: &mut R,
    id: i32,
) -> Result<BookEntity, String> {
    repository.get(id)
}