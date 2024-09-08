use crate::books::data::model::book::{BookModel, NewBookModel};
use core_diesel::tables::schema::books::dsl::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use diesel::{ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl};

pub struct BooksDao {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl BooksDao {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        BooksDao { pool }
    }

    pub fn get_books(&self) -> Result<Vec<BookModel>, Error> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let query_result = books.load::<BookModel>(&mut conn);

        match query_result {
            Ok(data) => Result::Ok(data),
            Err(e) => Err(e),
        }
    }

    pub fn create_book(&self, book: NewBookModel) -> Result<BookModel, Error> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let query_result = diesel::insert_into(books)
            .values(book)
            .get_result::<BookModel>(&mut conn);

        match query_result {
            Ok(data) => Result::Ok(data),
            Err(_) => Err(Error::NotFound),
        }
    }

    pub fn get_book(&self, book_id: i32) -> Result<BookModel, Error> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let query_result = books.find(book_id).first(&mut conn);

        match query_result {
            Ok(book) => Result::Ok(book),
            Err(e) => Err(e),
        }
    }

    pub fn delete_book(&self, book_id: i32) -> Result<(), String> {
        let mut conn = self
            .pool
            .get()
            .expect("couldn't get db connection from pool");

        let query_result = diesel::delete(books.filter(id.eq(book_id))).execute(&mut conn);

        match query_result {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string()),
        }
    }
}
