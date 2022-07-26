use strsim::*;

// TODO: Add tests to check what method of comparasion gives better results.
pub fn compare_similarity(name1: &str, name2: &str) -> f32 {
    jaro_winkler(name1.to_lowercase().as_str(), name2.to_lowercase().as_str()) as f32
}

/// Returns (discount_value, percentage_discount) if we have price_sales.
///
/// # Example
///
/// price_standard = 30;
/// price_sales = 15;
/// (15.0, 50)
pub fn discount(price_standard: f32, price_sales: Option<f32>) -> Option<(f32, u8)> {
    if let Some(price_sales) = price_sales {
        let discount: u8 = (price_sales / price_standard).round() as u8 * 100;
        let discount_value: f32 = price_standard - price_sales;
        return Some((discount_value, discount));
    }
    None
}
