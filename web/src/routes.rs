use crate::{establish_connection, params::QueryParams};
use actix_web::{
    HttpResponse, Responder, get, post,
    web::{Json, Query},
};
use database::{
    errors::DatabaseError,
    models::{BalancePeriod, Ride},
    queries::{get_balance::get_balance, post_ingest::post_ingest},
};

#[post("/ingest")]
pub async fn ingest(data: Json<Vec<Ride>>) -> impl Responder {
    let mut conn = establish_connection();
    println!("Handler called - received data: {:?}", data);
    let data: Vec<Ride> = data.into_inner();

    println!("Data extracted: {:?}", data);

    match post_ingest(&mut conn, data).await {
        Ok(result) => {
            println!("Success: {:?}", result);
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            println!("Database error: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Database error: {}", e))
        }
    }
}

#[get("/balances")]
pub async fn balances(params: Query<QueryParams>) -> impl Responder {
    let mut conn = establish_connection();
    let _period: BalancePeriod = params.clone().into_inner().period;
    let _driver_id: i32 = params.into_inner().driver_id;
    match get_balance(&mut conn, _driver_id, _period).await {
        Ok(balance) => {
            let tax_amount = financial_service::DriverFinancials::new(balance);
            HttpResponse::Ok().json(tax_amount.calculate_net_balance())
        }
        Err(DatabaseError::NoDriverFound) => {
            HttpResponse::NotFound().json("No Driver found for the specified ID")
        }
        Err(DatabaseError::NoRidesFound) => {
            HttpResponse::NoContent().json("No Ride found for the specified Driver and Period")
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}
