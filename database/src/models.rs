use chrono::NaiveDateTime;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, PartialEq)]
#[diesel(table_name = crate::schema::drivers)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Driver {
    pub id: i32,
    pub name: String,
}

#[derive(
    Serialize,
    Deserialize,
    Queryable,
    Selectable,
    Identifiable,
    Associations,
    Debug,
    PartialEq,
    AsChangeset,
)]
#[diesel(table_name = crate::schema::rides)]
#[diesel(belongs_to(Driver, foreign_key = driver_id))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Ride {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub driver_id: i32,
    pub amount: f64,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = crate::schema::rides)]
pub struct RideForm {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub driver_id: i32,
    pub amount: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum BalancePeriod {
    Monthly,
    Weekly,
    Daily,
}
