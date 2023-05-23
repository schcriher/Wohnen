// @generated automatically by Diesel CLI.

diesel::table! {
    houses (id) {
        id -> Integer,
        kind -> Text,
        street -> Text,
        number -> Integer,
        floor -> Integer,
        postcode -> Text,
        rooms -> Integer,
        baths -> Integer,
        area -> Float,
    }
}
