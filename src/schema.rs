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
        user_id -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        name -> Text,
        password -> Text,
        email -> Text,
    }
}

joinable!(todo_item -> todo_list (list_id));
joinable!(todo_list -> users (user_id));

allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
    users,
);
