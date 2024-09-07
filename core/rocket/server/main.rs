use std::sync::Arc;
use rocket::serde::json::Json;
use rocket::{get, launch, post, routes, State};

use diesel::r2d2::{ConnectionManager, Pool};
use rocket::response::status::Custom;
use rocket::tokio::sync::Mutex;
use core_diesel::connection::main::establish_connection;
use features::books::data::repository::book_repository_impl::BooksRepositoryImpl;
use features::books::domain::entity::book::BookEntity;
use features::books::domain::usecases::create_book::create_book;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/books", format = "json", data = "<book_dto>")]
async fn create_book_post(
    book_dto: Json<BookEntity>,
    books_repository_impl: &State<Arc<Mutex<BooksRepositoryImpl>>>
) -> Result<Json<BookEntity>, Custom<String>> {
    let mut book_repository = books_repository_impl.lock().await;

    let created_book = create_book(
        &mut *book_repository,
        book_dto.into_inner(),
    ).await;

    match created_book {
        Ok(book) => Ok(Json(book)),
        Err(e) => Err(
            Custom(
                rocket::http::Status::BadRequest,
                format!("Error creating book: {:?}", e)
            )
        )
    }
}

#[launch]
fn rocket() -> _ {
    let manager = establish_connection();
    let pool = Pool::builder().build(manager).unwrap();

    let book_repository = Arc::new(
        Mutex::new(
            BooksRepositoryImpl::new(pool)
        )
    );

    // manage is used to pass the book_repository to the Rocket application
    rocket::build()
        .manage(book_repository)
        .mount("/", routes![index, create_book_post])
}