use crate::errors::DatabaseError;
use crate::establish_connection;
use crate::models::Ride;
use crate::schema::rides::dsl::*;
use diesel::prelude::*;

pub async fn post_ingest(
    conn: &mut SqliteConnection,
    data: Vec<Ride>,
) -> Result<usize, DatabaseError> {
    Ok(diesel::insert_into(rides).values(&data).execute(conn)?)
}
