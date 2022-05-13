create table users (
  id serial primary key,
  name text not null unique,
  password text not null,
  email text not null unique
);


create table todo_list (
    id serial primary key,
    title varchar(150) not null,
    updated timestamp not null,
    user_id integer not null,
    foreign key (user_id) references users(id) on delete cascade
);

create table todo_item (
    id serial primary key,
    title varchar(150) not null,
    checked boolean not null default false,
    updated timestamp not null,
    list_id integer not null,
    foreign key (list_id) references todo_list(id) on delete cascade
);