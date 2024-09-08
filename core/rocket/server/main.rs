use rocket::serde::json::Json;
use rocket::{delete, get, launch, post, routes, State};
use std::sync::Arc;

use core_diesel::connection::main::establish_connection;
use diesel::r2d2::Pool;
use features::books::data::repository::book_repository_impl::BooksRepositoryImpl;
use features::books::domain::entity::book::BookEntity;
use features::books::domain::usecases::create_book::create_book;
use features::books::domain::usecases::delete_book::delete_book;
use features::books::domain::usecases::get_book::get_book;
use features::books::domain::usecases::get_books::get_books;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::tokio::sync::Mutex;

#[get("/books")]
async fn get_books_req(
    books_repository_impl: &State<Arc<Mutex<BooksRepositoryImpl>>>,
) -> Result<Json<Vec<BookEntity>>, Custom<String>> {
    let mut book_repository = books_repository_impl.lock().await;

    let fetched_books = get_books(&mut *book_repository);

    match fetched_books {
        Ok(books) => Ok(Json(books)),
        Err(e) => Err(Custom(
            Status::NotFound,
            format!("Error getting books: {:?}", e),
        )),
    }
}

#[post("/books", format = "json", data = "<book_data>")]
async fn create_book_req(
    book_data: Json<BookEntity>,
    books_repository_impl: &State<Arc<Mutex<BooksRepositoryImpl>>>,
) -> Result<Json<BookEntity>, Custom<String>> {
    let mut book_repository = books_repository_impl.lock().await;

    let created_book = create_book(&mut *book_repository, book_data.into_inner());

    match created_book {
        Ok(book) => Ok(Json(book)),
        Err(e) => Err(Custom(
            Status::BadRequest,
            format!("Error creating book: {:?}", e),
        )),
    }
}

#[get("/books/<id>")]
async fn get_book_req(
    id: i32,
    books_repository_impl: &State<Arc<Mutex<BooksRepositoryImpl>>>,
) -> Result<Json<BookEntity>, Custom<String>> {
    let mut book_repository = books_repository_impl.lock().await;

    let fetched_book = get_book(&mut *book_repository, id);

    match fetched_book {
        Ok(book) => Ok(Json(book)),
        Err(e) => Err(Custom(
            Status::NotFound,
            format!("Error getting book: {:?}", e),
        )),
    }
}

#[delete("/books/<id>")]
async fn delete_book_req(
    id: i32,
    books_repository_impl: &State<Arc<Mutex<BooksRepositoryImpl>>>,
) -> Result<Status, Custom<String>> {
    let mut book_repository = books_repository_impl.lock().await;

    match delete_book(&mut *book_repository, id) {
        Ok(_) => Ok(Status::NoContent),
        Err(e) => Err(Custom(
            Status::NotFound,
            format!("Error deleting book: {:?}", e),
        )),
    }
}

#[launch]
fn rocket() -> _ {
    let manager = establish_connection();
    let pool = Pool::builder().build(manager).unwrap();

    let book_repository = Arc::new(Mutex::new(BooksRepositoryImpl::new(pool)));

    // manage is used to pass the book_repository to the Rocket application
    rocket::build().manage(book_repository).mount(
        "/",
        routes![
            get_books_req,
            get_book_req,
            create_book_req,
            delete_book_req
        ],
    )
}
