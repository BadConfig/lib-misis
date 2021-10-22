use actix_web_dev::error::{
    Result,
    ErrorType,
    ApiError,
};
use serde::{Serialize, Deserialize};
use diesel::prelude::*;
use diesel::pg::PgConnection;
use super::*;

use crate::schema::{
    users,
    book_orders, 
    books,
    promo_codes,
};

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct SId {
    pub id: String,
}

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Id {
    pub id: i64,
}

#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Books {
    pub id: i64,
    pub name: String,
    pub unique_id: String,
    pub booking_per_day: i64,
    pub creation_datetime: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize,Clone)]
pub struct OrderNew {
    pub book_id: String,
    pub users_id: String,
    pub from: chrono::NaiveDate,
    pub to: chrono::NaiveDate,
    pub promo: String,
}

#[derive(Serialize,Deserialize,Clone,Insertable)]
#[table_name="books"]
pub struct BooksNew {
    pub name: String,
    pub unique_id: String,
    pub booking_per_day: i64,
}

#[derive(Serialize,Deserialize,Clone,Insertable)]
#[table_name="users"]
pub struct UsersNew {
    pub name: String,
    pub surname: String,
    pub patronymic: String,
    pub phone: String,
    pub unique_id: String,
}

#[derive(Serialize,Deserialize,Clone,AsChangeset)]
#[table_name="users"]
pub struct UsersMut {
    pub id: i64,
    pub name: Option<String>,
    pub surname: Option<String>,
    pub patronymic: Option<String>,
    pub phone: Option<String>,
}
#[derive(Serialize,Deserialize,Clone,Queryable)]
pub struct Users {
    pub id: i64,
    pub name: String,
    pub surname: String,
    pub patronymic: String,
    pub phone: String,
    pub unique_id: String,
    pub creation_datetime: chrono::NaiveDateTime,
}

#[derive(Serialize,Deserialize,Clone,Insertable)]
#[table_name="book_orders"]
pub struct BookOrdersNew {
    pub book_id: String,
    pub users_id: String,
    pub from: chrono::NaiveDate,
    pub to: chrono::NaiveDate,
    pub total_price: i64,
}

use diesel::sql_types::{
    Bigint,
    Varchar,
    Date,
    Timestamp,
    Bool,
};

#[derive(Serialize,Deserialize,Clone,QueryableByName)]
pub struct BookOrders {
    #[sql_type="Varchar"]
    pub book_name: String,
    #[sql_type="Bigint"]
    pub id: i64,
    #[sql_type="Varchar"]
    pub book_id: String,
    #[sql_type="Varchar"]
    pub users_id: String,
    #[sql_type="Date"]
    pub from: chrono::NaiveDate,
    #[sql_type="Date"]
    pub to: chrono::NaiveDate,
    #[sql_type="Bigint"]
    pub total_price: i64,
    #[sql_type="Bool"]
    pub is_payed: bool,
    #[sql_type="Timestamp"]
    pub creation_datetime: chrono::NaiveDateTime,
}


#[derive(Serialize,Deserialize,Clone)]
pub struct BookOp {
    pub orders: Vec<BookOrders>,
    pub user: Users,
}

impl Books {
    pub async fn new(
        creds: &BooksNew, 
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::insert_into(books::table)
            .values(creds)
            .execute(conn)?;
        Ok(())
    }
    pub async fn get(
        conn: &PgConnection,
    ) -> Result<Vec<Books>> {
        let r = books::table
            .get_results(conn)?;
        Ok(r)
    }
}

impl Users {

    pub async fn get(
        conn: &PgConnection,
    ) -> Result<Vec<BookOp>> {
        let mut r = Vec::<BookOp>::new();
        let u = users::table
            .get_results::<Users>(conn)?;
        for i in u {
            let bks  = diesel::sql_query("select * from wide_orders where users_id=$1")
                    .bind::<Varchar,_>(i.unique_id.clone())
                .get_results::<BookOrders>(conn)?;
            r.push(
                BookOp {
                    orders: bks,
                    user: i,
                }
            );
        } 
        Ok(r)
    }

    pub async fn new_order(
        creds: &OrderNew, 
        conn: &PgConnection,
    ) -> Result<()> {
        let disc = promo_codes::table
            .filter(promo_codes::name.eq(&creds.promo))
            .select(promo_codes::quantity)
            .get_results(conn)?; 
        let disc = match disc.len() {
            0 => 0,
            _ => disc[0],
        };

        let book = books::table
            .filter(books::unique_id.eq(&creds.book_id))
            .select(books::booking_per_day)
            .get_result::<i64>(conn)?;

        let total_price = book * (creds.to - creds.from).num_days() * (100 - disc) / 100;

        let bo = BookOrdersNew {
            book_id: creds.book_id.clone(),
            users_id: creds.users_id.clone(),
            from: creds.from,
            to: creds.to,
            total_price,
        };

        diesel::insert_into(book_orders::table)
            .values(bo)
            .execute(conn)?;
        Ok(())
    }

    pub async fn new(
        creds: &UsersNew, 
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::insert_into(users::table)
            .values(creds)
            .execute(conn)?;
        Ok(())
    }

    pub async fn from_id(
        id: i64,
        conn: &PgConnection,
    ) -> Result<Self> {
        let r = users::table
            .filter(users::id.eq(id))
            .get_result::<Self>(conn)?;
        Ok(r)
    }

    pub async fn set_payed(
        booking_id: i64,
        conn: &PgConnection,
    ) -> Result<()> {
        diesel::update(book_orders::table)
            .filter(book_orders::id.eq(booking_id))
            .set(book_orders::is_payed.eq(true))
            .execute(conn)?;
        Ok(())
    }

    pub async fn get_books(
        user_id: String,
        conn: &PgConnection,
    ) -> Result<Vec<BookOrders>> {
        let r = diesel::sql_query("select * from wide_orders where users_id=$1")
                .bind::<Varchar,_>(user_id)
            .get_results::<BookOrders>(conn)?;
        Ok(r)
    }

    pub async fn set(
        instance: &UsersMut,
        conn: &PgConnection,
    ) -> Result<Self> {
        let r = diesel::update(users::table
            .filter(users::id.eq(instance.id)))
            .set(instance)
            .get_result::<Self>(conn)?;
        Ok(r)
    }
}
