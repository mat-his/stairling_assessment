use diesel::SqliteConnection;

pub mod params;
pub mod routes;

#[cfg(test)]
pub fn establish_connection() -> SqliteConnection {
    use database::establish_connection;

    establish_connection()
}

#[cfg(not(test))]
pub fn establish_connection() -> SqliteConnection {
    use database::establish_test_connection;

    establish_test_connection()
}
