-- Your SQL goes here

create table users (
  id serial primary key,
  name text not null unique,
  password text not null,
  email text not null unique,
  todo_id integer,
  foreign key (todo_id) references todo_list(id)
);
