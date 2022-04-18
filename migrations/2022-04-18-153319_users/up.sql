-- Your SQL goes here

create table users (
  id serial primary key,
  name text not null,
  password text not null,
  email text not null,
  todo_id integer not null,
  foreign key (todo_id) references todo_list(id)
);
