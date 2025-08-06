// @generated automatically by Diesel CLI.

diesel::table! {
    drivers (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::table! {
    rides (id) {
        id -> Integer,
        driver_id -> Integer,
        amount -> Double,
        created_at -> Timestamp,
    }
}

diesel::joinable!(rides -> drivers (driver_id));

diesel::allow_tables_to_appear_in_same_query!(
    drivers,
    rides,
);
