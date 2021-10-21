use actix_web::{
    web, http, dev, guard,
    App, HttpResponse, client::Client,
    HttpServer, HttpRequest, Responder,
};
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

use actix_web_dev::error::{
    Result,
};

use super::db::{
    Users,
    UsersNew,
    Books,
    BooksNew,
    OrderNew,
    Id,
};

pub fn users_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/new_book", web::post().to(new_book))
        .route("/new_user", web::post().to(new_user))
        .route("/add_order", web::post().to(add_order))
        .route("/users", web::post().to(users))
        .route("/books", web::post().to(books))
        .route("/user_by_id", web::post().to(user_by_id));
}

pub async fn new_book(
    form: web::Json<BooksNew>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Books::new(&form, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn new_user(
    form: web::Json<UsersNew>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Users::new(&form, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn add_order(
    form: web::Json<OrderNew>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Users::new_order(&form, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn user_by_id(
    form: web::Json<Id>,
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Users::from_id(form.id, &conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn books(
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Books::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}

pub async fn users(
    conn: web::Data<DbPool>,
) -> Result<HttpResponse> {
    let conn = conn.get()?;
    let r = Users::get(&conn).await?;
    Ok(HttpResponse::Ok().json(r))
}
