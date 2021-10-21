create table users (
    id BIGSERIAL primary key,
    name varchar not null,
    surname varchar not null,
    patronymic varchar not null,
    phone varchar not null,
    unique_id varchar not null unique,
    creation_datetime timestamp not null default current_timestamp
);

create table books (
    id BIGSERIAL primary key,
    name varchar not null,
    unique_id varchar not null unique,
    booking_per_day bigint not null,
    creation_datetime timestamp not null default current_timestamp
);

create table book_orders (
    id BIGSERIAL primary key,
    book_id varchar not null,
    users_id varchar not null,
    "from" date not null,
    "to" date not null,
    total_price bigint not null,
    is_payed bool not null default false,
    creation_datetime timestamp not null default current_timestamp
);

create table promo_codes (
    id BIGSERIAL primary key,
    name varchar not null,
    quantity bigint not null
);
