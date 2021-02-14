table! {
    flavors (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
    }
}

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    flavors,
    posts,
);
