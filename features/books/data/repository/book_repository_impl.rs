use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::books::data::data_sources::local::dao::books_dao::BooksDao;
use crate::books::data::model::book::{NewBookModel};
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
    fn create(&mut self, book: BookEntity) -> Result<BookEntity, String> {
        let book_model = NewBookModel::from(book);
        let mut dao = &mut self.dao;

        match dao.create_book(book_model) {
            // book.into will convert BookEntity to BookModel
            Ok(book) => Ok(book.into()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn get(&self, id: i32) -> Result<BookEntity, String> {
        let mut dao = &self.dao;
        let book = dao.get_book(id);

        match book {
            Ok(book) => Ok(book.into()),
            Err(e) => Err(e.to_string()),
        }
    }

    fn update(&self, _book: &BookEntity) -> Result<BookEntity, String> {
        todo!()
    }

    fn delete(&self, id: i32) -> Result<(), String> {
        let mut dao = &self.dao;
        let delete_result = dao.delete_book(id);

        match delete_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}