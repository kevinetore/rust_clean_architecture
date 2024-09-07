use diesel::{PgConnection, RunQueryDsl};
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::result::Error;
use core_diesel::tables::schema::books::dsl::books;
use crate::books::data::model::book::BookModel;

pub struct BooksDao {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl BooksDao {
    pub fn new(pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        BooksDao { pool }
    }

    pub async fn create_book<'a>(&'a self, book: &'a BookModel) -> Result<&BookModel, Error> {
        let mut conn = self.pool.get().expect("couldn't get db connection from pool");

        let query_result = diesel::insert_into(books)
            .values(book)
            .execute(&mut conn);

        match query_result {
            Ok(_) => Result::Ok(book),
            Err(_) => Err(Error::NotFound)
        }
    }
}