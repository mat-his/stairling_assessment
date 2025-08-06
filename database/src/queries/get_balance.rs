use chrono::{Duration, Local};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};

use crate::errors::DatabaseError;
use crate::models::{BalancePeriod, Driver};
use crate::schema::drivers::dsl::*;
use crate::schema::rides::dsl::*;

pub async fn get_balance(
    conn: &mut SqliteConnection,
    _driver_id: i32,
    period: BalancePeriod,
) -> Result<f64, DatabaseError> {
    let now = Local::now().naive_local();

    match drivers
        .find(_driver_id)
        .select(Driver::as_select())
        .first(conn)
    {
        Ok(_) => (),
        Err(_) => return Err(DatabaseError::NoDriverFound),
    }

    let results = match period {
        BalancePeriod::Daily => {
            let day_ago = now - Duration::days(1);
            rides
                .filter(driver_id.eq(_driver_id))
                .filter(created_at.ge(day_ago))
                .filter(created_at.le(now - Duration::days(2)))
                .select(amount)
                .load(conn)
        }
        BalancePeriod::Weekly => {
            let week_ago = now - Duration::weeks(1);
            rides
                .filter(driver_id.eq(_driver_id))
                .filter(created_at.ge(week_ago))
                .filter(created_at.le(now - Duration::weeks(2)))
                .select(amount)
                .load(conn)
        }
        BalancePeriod::Monthly => {
            let month_ago = now - Duration::days(30); // Approximation for a month
            rides
                .filter(driver_id.eq(_driver_id))
                .filter(created_at.ge(month_ago))
                .filter(created_at.le(now - Duration::days(60)))
                .select(amount)
                .load(conn)
        }
    }?;

    if results.is_empty() {
        return Err(DatabaseError::NoRidesFound);
    }

    Ok(results.iter().sum())
}
