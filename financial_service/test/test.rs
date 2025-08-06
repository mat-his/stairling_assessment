#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_net_balance() {
        let financials = DriverFinancials::new(1000.0);
        let net_balance = financials.calculate_net_balance();
        assert_eq!(
            net_balance,
            1000.0 * (1.0 - 0.15) * (1.0 - 0.20) * (1.0 - 0.20)
        );
    }
}
