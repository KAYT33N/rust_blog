// @generated automatically by Diesel CLI.

diesel::table! {
    access_tokens (id) {
        id -> Int4,
        hashed -> Bpchar,
        user_id -> Int4,
        age -> Int4,
        created_at -> Timestamp,
    }
}

diesel::table! {
    posts (id) {
        id -> Int4,
        user_id -> Int4,
        body -> Text,
        created_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        password -> Bpchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    access_tokens,
    posts,
    users,
);
