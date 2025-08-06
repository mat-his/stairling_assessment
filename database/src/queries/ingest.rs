use crate::establish_connection;
use crate::models::RideForm;
use crate::schema::rides::dsl::*;
use diesel::prelude::*;

pub fn ingest(data: Vec<RideForm>) -> usize {
    let connection = &mut establish_connection();

    diesel::insert_into(rides)
        .values(&data)
        .execute(connection)
        .expect("Error saving rides")
}
