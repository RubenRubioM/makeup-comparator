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
/// The normalized value between 0-10
///
/// # Example
/// let rating = normalized_rating(25.0, 50.0);
/// assert_eq!(rating, 5.0);
#[allow(dead_code)]
pub fn normalized_rating(rating: f32, max_rating: f32) -> f32 {
    rating * 10.0 / max_rating
}

/// Enumeration of possible errors when using the scraper crate.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HtmlSearchError {
    ElementNotFound(String),
    AttributeNotFound(String),
}

/// Debug implementation for HtmlSearchError to print when unwrapping.
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

/// Returns the inner html for the selector provided in the element
/// # Arguments
/// element - The HTML element
/// selector - The css selector
///
/// # Returns
/// String - if found
/// HtmlSearchError::ElementNotFound - if not found the selector
pub fn inner_html_value(element: &ElementRef, selector: &str) -> Result<String, HtmlSearchError> {
    match element
        .select(&scraper::Selector::parse(selector).unwrap())
        .next()
    {
        Some(value) => Ok(value.inner_html()),
        None => Err(HtmlSearchError::ElementNotFound(selector.to_string())),
    }
}

/// Returns the value for an attribute inside the selector provided in the element
/// # Arguments
/// element - The HTML element
/// selector - The css selector
/// attribute - The html attribute
///
/// # Returns
/// String - if found
/// HtmlSearchError::ElementNotFound - if not found the selector
/// HtmlSearchError::AttributeNotFound - if not found the attribute
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

/// Returns the value for an attribute inside the selector provided in the element
/// # Arguments
/// element - The HTML element
/// selector - The css selector
/// attribute - The html attribute
///
/// # Returns
/// String - if found
/// HtmlSearchError::ElementNotFound - if not found the selector
/// HtmlSearchError::AttributeNotFound - if not found the attribute
pub fn has_html_selector(element: &ElementRef, selector: &str) -> bool {
    element
        .select(&scraper::Selector::parse(selector).unwrap())
        .next()
        .is_some()
}

#[cfg(test)]
mod tests {
    use scraper::Html;

    use super::*;

    #[test]
    /// Tests discount() with price_sales.
    fn test_discount_with_price_sales() {
        let (discount_value, percentage_discount) = discount(50.0, Some(25.0)).unwrap();
        assert_eq!(discount_value, 25.0);
        assert_eq!(percentage_discount, 50);
    }

    #[test]
    /// Discount method return None if there is not price_sales.
    #[should_panic]
    fn test_discount_without_price_sales() {
        let (_discount_value, _percentage_discount) = discount(50.0, None).unwrap();
    }

    /// Tests the parsing between money string and return the value in floating pointer.
    #[test]
    fn test_parse_price_string() {
        assert_eq!(38.95, parse_price_string("38,95 €".to_string()));
        assert_eq!(38.95, parse_price_string("38.95 $".to_string()));
        assert_eq!(38, parse_price_string("38 $".to_string()));
        assert_eq!(38, parse_price_string("38€".to_string()));
        assert_eq!(38.3, parse_price_string("38,3€".to_string()));
    }

    /// Tests if the rating is properly normalized between 0-5.
    #[test]
    fn test_normalized_rating() {
        assert_eq!(2.0, normalized_rating(20.0, 100.0));
        assert_eq!(10.0, normalized_rating(5.0, 5.0));
        assert_eq!(1.0, normalized_rating(1.0, 10.0));
    }

    /// Tests if the inner html value is properly returned.
    #[test]
    fn test_inner_html_value() {
        let html = r#"
            <!DOCTYPE html>
            <meta charset="utf-8">
            <title>Hello, world!</title>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        let document = Html::parse_document(html);
        let element = document.root_element();
        let ok_result = inner_html_value(&element, "h1");
        assert!(Result::is_ok(&ok_result));
        assert_eq!(ok_result.unwrap(), "Hello, <i>world!</i>");

        let element_not_found_result = inner_html_value(&element, "h2");
        assert!(Result::is_err(&element_not_found_result));
        assert_eq!(
            element_not_found_result.unwrap_err(),
            HtmlSearchError::ElementNotFound("h2".to_string())
        );
    }

    /// Tests if the attribute html value is properly returned.
    #[test]
    fn test_attribute_html_value() {
        let html = r#"
            <!DOCTYPE html>
            <meta charset="utf-8">
            <title>Hello, world!</title>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        let document = Html::parse_document(html);
        let element = document.root_element();
        let ok_result = attribute_html_value(&element, "h1", "class");
        assert!(Result::is_ok(&ok_result));
        assert_eq!(ok_result.unwrap(), "foo");

        let element_not_found_result = attribute_html_value(&element, "h2", "class");
        assert!(Result::is_err(&element_not_found_result));
        assert_eq!(
            element_not_found_result.clone().unwrap_err(),
            HtmlSearchError::ElementNotFound("h2".to_string())
        );
        println!(
            "Testing Debug trait for HtmlSearchError::ElementNotFound: {:?}",
            element_not_found_result.unwrap_err()
        );

        let attribute_not_found_result = attribute_html_value(&element, "h1", "id");
        assert!(Result::is_err(&attribute_not_found_result));
        assert_eq!(
            attribute_not_found_result.clone().unwrap_err(),
            HtmlSearchError::AttributeNotFound("id".to_string())
        );
        println!(
            "Testing Debug trait for HtmlSearchError::AttributeNotFound: {:?}",
            attribute_not_found_result.unwrap_err()
        );
    }

    /// Tests if the selector is properly found.
    #[test]
    fn test_has_html_selector() {
        let html = r#"
            <!DOCTYPE html>
            <meta charset="utf-8">
            <title>Hello, world!</title>
            <h1 class="foo">Hello, <i>world!</i></h1>
        "#;

        let document = Html::parse_document(html);
        let element = document.root_element();
        assert!(has_html_selector(&element, "h1"));
        assert!(!has_html_selector(&element, "h2"));
    }
}
