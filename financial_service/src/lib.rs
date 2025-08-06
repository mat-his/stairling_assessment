//! A library for calculating the balance due to drivers after applying various deductions.

/// Represents the financial details for a driver over a given period.
#[derive(Debug)]
pub struct DriverFinancials {
    /// The gross amount earned by the driver.
    pub gross_amount: f64,
}

impl DriverFinancials {
    /// Creates a new `DriverFinancials` instance with the given gross amount.
    pub fn new(gross_amount: f64) -> Self {
        DriverFinancials { gross_amount }
    }
    /// Calculates the net balance due to the driver after applying all deductions.
    pub fn calculate_net_balance(&self) -> f64 {
        let amount_after_commission = self.apply_service_commission(self.gross_amount);
        let amount_after_vat = self.apply_vat(amount_after_commission);
        self.apply_urssaf(amount_after_vat)
    }

    /// Applies the service commission to the given amount.
    fn apply_service_commission(&self, amount: f64) -> f64 {
        let service_commission_rate = 0.15;
        amount - (amount * service_commission_rate)
    }

    /// Applies the VAT to the given amount.
    fn apply_vat(&self, amount: f64) -> f64 {
        let vat_rate = 0.20;
        amount - (amount * vat_rate)
    }

    /// Applies the Urssaf deductions to the given amount.
    fn apply_urssaf(&self, amount: f64) -> f64 {
        let urssaf_rate = 0.20;
        amount - (amount * urssaf_rate)
    }
}
