use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::books::data::data_sources::local::dao::books_dao::BooksDao;
use crate::books::data::model::book::BookModel;
use crate::books::domain::entity::book::BookEntity;
use crate::books::domain::repository::book_repository::BookRepository;

pub struct BooksRepositoryImpl {
    dao: BooksDao,
}

impl BooksRepositoryImpl {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        BooksRepositoryImpl {
            dao: BooksDao::new(pool),
        }
    }
}

impl BookRepository for BooksRepositoryImpl {
    async fn create(&mut self, book: &BookEntity) -> Result<BookEntity, String> {
        let book_model = BookModel::from(book);
        let mut dao = &mut self.dao;

        match dao.create_book(&book_model).await {
            // book.into will convert BookEntity to BookModel
            Ok(book) => Ok(book.into()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn read(&self, _id: i32) -> Result<BookEntity, String> {
        todo!()
    }

    fn update(&self, _book: &BookEntity) -> Result<BookEntity, String> {
        todo!()
    }

    fn delete(&self, _id: i32) -> Result<BookEntity, String> {
        todo!()
    }
}