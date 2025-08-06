use database::models::BalancePeriod;
use serde::Deserialize;

// Helper struct to deserialize the query parameter
#[derive(Debug, Deserialize, Clone)]
pub struct QueryParams {
    pub period: BalancePeriod,
    pub driver_id: i32,
}
