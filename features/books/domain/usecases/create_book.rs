use crate::books::domain::entity::book::BookEntity;
use crate::books::domain::repository::book_repository::BookRepository;

pub fn create_book<R: BookRepository>(
    repository: &mut R,
    book: BookEntity,
) -> Result<BookEntity, String> {
    let book = BookEntity { ..book };

    repository.create(book)
}
