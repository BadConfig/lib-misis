table! {
    use diesel::sql_types::*;

    book_orders (id) {
        id -> Int8,
        book_id -> Varchar,
        users_id -> Varchar,
        from -> Date,
        to -> Date,
        total_price -> Int8,
        is_payed -> Bool,
        creation_datetime -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    books (id) {
        id -> Int8,
        name -> Varchar,
        unique_id -> Varchar,
        booking_per_day -> Int8,
        creation_datetime -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;

    promo_codes (id) {
        id -> Int8,
        name -> Varchar,
        quantity -> Int8,
    }
}

table! {
    use diesel::sql_types::*;

    users (id) {
        id -> Int8,
        name -> Varchar,
        surname -> Varchar,
        patronymic -> Varchar,
        phone -> Varchar,
        unique_id -> Varchar,
        creation_datetime -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    book_orders,
    books,
    promo_codes,
    users,
);
