use std::str::FromStr;

use scraper::ElementRef;
use strsim::*;

pub fn compare_similarity(name1: &str, name2: &str) -> f32 {
    jaro_winkler(name1.to_lowercase().as_str(), name2.to_lowercase().as_str()) as f32
}

/// Returns (discount_value, percentage_discount) if we have price_sales.
///
/// # Arguments
/// price_standard - The normal price.
/// price_sales - The price with the discount.
///
/// # Returns
/// An Option of a tuple with the discount value and the percentage
///
/// # Example
/// let (discount_value, percentage_discount) = helper::discount(30.0, Some(15.0)).unwrap();
/// assert_eq!(discount_value, 15.0);
/// assert_eq!(percentage_discount, 50);
#[allow(dead_code)]
pub fn discount(price_standard: f32, price_sales: Option<f32>) -> Option<(f32, u8)> {
    if let Some(price_sales) = price_sales {
        let discount_value: f32 = price_standard - price_sales;
        let discount: u8 = ((price_sales / price_standard) * 100.0).round() as u8;
        return Some((discount_value, discount));
    }
    None
}

/// Returns the price in floating number.
///
/// # Arguments
/// price - The price in string format.
///
/// # Returns
/// T - The value in value.
///
/// # Example
/// let price_string: String = String::from("38,95 €");
/// let price_float: f32 = parse_price_string(price_string);
/// assert_eq!(price_float, 38.95_f32);
pub fn parse_price_string<T: FromStr>(price: String) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let currency_characters: Vec<char> = vec!['€', '$'];
    price
        .replace(',', ".")
        .replace(&currency_characters[..], "")
        .trim()
        .to_string()
        .parse::<T>()
        .unwrap()
}

/// Returns the normalized value between 0-5
///
/// # Arguments
/// rating - The rating value.
/// max_rating - The maximum rating available
///
/// # Returns
/// The normalized value between 0-5
///
/// # Example
/// let rating = normalized_rating(20.0, 100.0);
/// assert_eq!(rating, 1.0);
#[allow(dead_code)]
pub fn normalized_rating(rating: f32, max_rating: f32) -> f32 {
    rating * 5.0 / max_rating
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HtmlSearchError {
    ElementNotFound(String),
    AttributeNotFound(String),
}

impl std::fmt::Debug for HtmlSearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HtmlSearchError::ElementNotFound(selector) => {
                write!(f, "selector: \"{selector}\" not found.")
            }
            HtmlSearchError::AttributeNotFound(attribute) => {
                write!(f, "attribute: \"{attribute}\" not found.")
            }
        }
    }
}

pub fn inner_html_value(element: &ElementRef, selector: &str) -> Result<String, HtmlSearchError> {
    match element
        .select(&scraper::Selector::parse(selector).unwrap())
        .next()
    {
        Some(value) => Ok(value.inner_html()),
        None => Err(HtmlSearchError::ElementNotFound(selector.to_string())),
    }
}

pub fn attribute_html_value(
    element: &ElementRef,
    selector: &str,
    attribute: &str,
) -> Result<String, HtmlSearchError> {
    match element
        .select(&scraper::Selector::parse(selector).unwrap())
        .next()
    {
        Some(value) => match value.value().attr(attribute) {
            Some(attribute_value) => Ok(attribute_value.to_string()),
            None => Err(HtmlSearchError::AttributeNotFound(attribute.to_string())),
        },
        None => Err(HtmlSearchError::ElementNotFound(selector.to_string())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// Tests discount() with price_sales.
    fn test_discount_with_price_sales() {
        let (discount_value, percentage_discount) = discount(50.0, Some(25.0)).unwrap();
        assert_eq!(discount_value, 25.0);
        assert_eq!(percentage_discount, 50);
    }

    #[test]
    #[should_panic]
    /// Discount method return None if there is not price_sales.
    fn test_discount_without_price_sales() {
        let (_discount_value, _percentage_discount) = discount(50.0, None).unwrap();
    }

    #[test]
    /// Tests the parsing between money string and return the value in floating pointer.
    fn test_parse_price_string() {
        assert_eq!(38.95, parse_price_string("38,95 €".to_string()));
        assert_eq!(38.95, parse_price_string("38.95 $".to_string()));
        assert_eq!(38, parse_price_string("38 $".to_string()));
        assert_eq!(38, parse_price_string("38€".to_string()));
        assert_eq!(38.3, parse_price_string("38,3€".to_string()));
    }

    #[test]
    /// Tests if the rating is properly normalized between 0-5.
    fn test_normalized_rating() {
        assert_eq!(1.0, normalized_rating(20.0, 100.0));
        assert_eq!(5.0, normalized_rating(5.0, 5.0));
        assert_eq!(1.0, normalized_rating(1.0, 5.0));
    }
}
