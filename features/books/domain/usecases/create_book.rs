use crate::books::domain::entity::book::BookEntity;
use crate::books::domain::repository::book_repository::BookRepository;

pub async fn create_book<R: BookRepository>(
    repository: &mut R,
    book: BookEntity,
) -> Result<BookEntity, String> {
    let book = BookEntity {
        id: 1,
        ..book
    };

    repository
        .create(&book)
        .await
}