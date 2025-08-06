use chrono::{Duration, Local, NaiveDateTime};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::establish_connection;
use crate::models::{BalancePeriod, Ride};
use crate::schema::rides::dsl::*;

#[derive(Error, Debug)]
pub enum BalanceError {
    #[error("Database error")]
    DatabaseError(#[from] diesel::result::Error),
}

pub fn get_balance(_driver_id: i32, period: BalancePeriod) -> Result<f64, BalanceError> {
    let connection = &mut establish_connection();
    let now = Local::now().naive_local();

    let results = match period {
        BalancePeriod::Daily => {
            let day_ago = now - Duration::days(1);
            rides
                .filter(driver_id.eq(_driver_id))
                .filter(created_at.ge(day_ago))
                .select(amount)
                .load(connection)
        }
        BalancePeriod::Weekly => {
            let week_ago = now - Duration::weeks(1);
            rides
                .filter(driver_id.eq(_driver_id))
                .filter(created_at.ge(week_ago))
                .select(amount)
                .load(connection)
        }
        BalancePeriod::Monthly => {
            let month_ago = now - Duration::days(30); // Approximation for a month
            rides
                .filter(driver_id.eq(_driver_id))
                .filter(created_at.ge(month_ago))
                .select(amount)
                .load(connection)
        }
    }?;

    Ok(results.iter().sum())
}
