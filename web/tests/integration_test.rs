use actix_web::{App, middleware, test};
use chrono::{Duration, Local, Utc};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use database::models::{BalancePeriod, Driver, Ride};
use database::queries::get_balance;
use database::schema::drivers::dsl::*;
use database::schema::rides::dsl::*;
use diesel::SqliteConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use lazy_static::lazy_static;
use web::establish_connection;
use web::routes::{balances, ingest};

lazy_static! {
    pub static ref NAIVE_DATETIME: NaiveDateTime = {
        let date = NaiveDate::from_ymd_opt(2023, 10, 1).expect("Invalid date");
        let time = NaiveTime::from_hms_opt(12, 0, 0).expect("Invalid time");
        NaiveDateTime::new(date, time)
    };
}
// Create a list of dummy rides
lazy_static! {
pub static ref DUMMY_RIDES: Vec<Ride> = vec![
    Ride {
        id: 1,
        driver_id: 1,
        amount: 100.0,
        created_at: Local::now().naive_local() - Duration::days(1),
    },
    Ride {
        id: 2,
        driver_id: 1,
        amount: 200.0,
        created_at:  Local::now().naive_local() - Duration::weeks(1),
    },
    Ride {
        id: 3,
        driver_id: 2,
        amount: 150.0,
        created_at:  Local::now().naive_local() - Duration::days(30),
    },
    // Add more dummy rides as needed
];
}

// Create a list of dummy rides
lazy_static! {
    pub static ref DUMMY_DRIVERS: Vec<Driver> = vec![
        Driver {
            id: 1,
            name: String::from("Gege")
        },
        Driver {
            id: 2,
            name: String::from("Boby")
        }
    ];
}

// Set up test database connection
pub async fn establish_test_connection() -> SqliteConnection {
    let mut conn = establish_connection();

    populate_test_data(&mut conn);

    conn
}

fn populate_test_data(conn: &mut SqliteConnection) {
    // Clear existing data
    diesel::delete(drivers).execute(conn).unwrap_or_default();
    diesel::delete(rides).execute(conn).unwrap_or_default();

    // Insert dummy drivers into the database
    diesel::insert_into(drivers)
        .values::<Vec<Driver>>(DUMMY_DRIVERS.to_vec())
        .execute(conn)
        .expect("Failed to insert dummy drivers");

    // Insert dummy rides into the database
    diesel::insert_into(rides)
        .values::<Vec<Ride>>(DUMMY_RIDES.to_vec())
        .execute(conn)
        .expect("Failed to insert dummy rides");
}

#[actix_web::test]
async fn test_balances_endpoint() {
    let mut conn = establish_test_connection().await;

    let res = get_balance::get_balance(&mut conn, 1, BalancePeriod::Daily).await;

    print!("{:?}", res);

    // Initialize your app for testing
    let app = test::init_service(App::new().service(balances)).await;

    // Create a test request
    let req = test::TestRequest::get()
        .uri("/balances?period=Daily&driver_id=1")
        .to_request();

    // Send the request to the app
    let response = test::call_service(&app, req).await;
    print!("my results {:?}", response.response());
    assert!(response.status().is_success());
}

// Test error handling for balances endpoint
#[actix_web::test]
async fn test_balances_endpoint_error() {
    let app = test::init_service(App::new().service(balances)).await;
    let request = test::TestRequest::get()
        .uri("/balances?period=Daily&driver_id=999") // Assuming 999 is an invalid driver_id
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404); // Expecting a NOT_FOUND error
}

// Test for the ingest endpoint
#[actix_web::test]
async fn test_ingest_endpoint() {
    let mut conn = establish_test_connection().await;

    match drivers.find(1).select(Driver::as_select()).first(&mut conn) {
        Ok(e) => print!("{:?}", e.name),
        Err(e) => print!("{:?}", e),
    }

    let app = test::init_service(
        App::new()
            .service(ingest)
            .wrap(middleware::Logger::default()),
    )
    .await;
    let request = test::TestRequest::post()
        .uri("/ingest")
        .set_json(vec![Ride {
            driver_id: 0,
            id: 4,
            amount: 50.0,
            created_at: Utc::now().naive_utc(),
        }])
        .to_request();

    let response = test::call_service(&app, request).await;
    print!("{:?}", response.response());
    assert!(response.status().is_success());
}
