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

create or replace view wide_orders as
    select
        b.name as book_name,
        bo.id,
        bo.book_id,
        bo.users_id,
        bo."from",
        bo."to",
        bo.total_price,
        bo.is_payed,
        bo.creation_datetime
    from
        book_orders bo left join books b on b.unique_id = bo.book_id;

insert into promo_codes (name, quantity) values ('1percent',1::bigint),('90percent',90::bigint);
