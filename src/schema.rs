table! {
    todo_item (id) {
        id -> Int4,
        title -> Varchar,
        checked -> Bool,
        list_id -> Int4,
    }
}

table! {
    todo_list (id) {
        id -> Int4,
        title -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
        email -> Text,
        todo_id -> Int4,
    }
}

joinable!(todo_item -> todo_list (list_id));
joinable!(users -> todo_list (todo_id));

allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
    users,
);
