// @generated automatically by Diesel CLI.

diesel::table! {
    pairs (id) {
        id -> Int4,
        #[max_length = 255]
        spanish -> Varchar,
        #[max_length = 255]
        english -> Varchar,
    }
}
