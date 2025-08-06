pub mod params;

use actix_web::{
    App, Either, HttpRequest, HttpResponse, HttpServer, Responder, get, post,
    web::{Form, Json, Query},
};
use database::{
    models::{BalancePeriod, RideForm},
    queries::get_balance::get_balance,
};

use crate::params::QueryParams;

#[get("/ingest")]
async fn ingest(data: Either<Json<Vec<RideForm>>, Form<Vec<RideForm>>>) -> impl Responder {
    let _data: Vec<RideForm> = data.into_inner();
    database::queries::ingest::ingest(_data);
    HttpResponse::Ok()
}

#[post("/balances")]
async fn balances(params: Query<QueryParams>) -> impl Responder {
    let _period: BalancePeriod = params.clone().into_inner().period;
    let _driver_id: i32 = params.into_inner().driver_id;
    match get_balance(_driver_id, _period) {
        Ok(balance) => {
            let tax_amount = financial_service::DriverFinancials::new(balance);
            HttpResponse::Ok().json(tax_amount.calculate_net_balance())
        }
        Err(e) => HttpResponse::InternalServerError().json(e.to_string()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(ingest).service(balances))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
