// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int8,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
