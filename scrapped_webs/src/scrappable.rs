//! Trait that defines the scrappable trait
use anyhow;
use scraper::{ElementRef, Html};
use thiserror;

use crate::product::{Product, Tone};

/// Enumeration of possible error when trying to search a product.
#[derive(thiserror::Error, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SearchError {
    #[error("timeout when doing the petition.")]
    Timeout,
    #[error("not found any result above the minimum similarity rate.")]
    NotEnoughSimilarity,
    #[error("not found any result.")]
    NotFound,
}

pub trait Scrappable {
    /// Try to find the product in the website.
    ///
    /// # Arguments
    /// name - The name of the product to find.
    /// # Returns
    /// Product - A vector with the similar products that matches the name.
    /// Box<dyn Error> - If couldn't find the product.
    fn look_for_products(&self, name: String) -> Result<Vec<Product>, anyhow::Error>;

    /// Returns the url of the products found.
    /// # Arguments
    /// document - The search page HTML document with some or none products found.
    /// name - The name provided by the user to find.
    /// # Returns
    /// Ok - Vector with the urls found in the search page.
    /// Err - Search error.
    fn search_results_urls(
        &self,
        document: &Html,
        name: &str,
    ) -> Result<Vec<String>, anyhow::Error>;

    /// Creates and initialize the product object.
    ///
    /// # Arguments
    /// document - The HTML document for the product to create.
    /// # Returns
    /// Product - The product created based on this HTML webpage.
    fn create_product(document: &Html) -> Product;

    /// Creates and initialize a tone for a product.
    ///
    /// # Arguments
    /// element - The HTML element containing the information fo the tone (could be the hole website).
    /// # Returns
    /// Tone - The individual tone.
    fn create_tone(element: &ElementRef) -> Tone;
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    /// Test the search error display implementation.
    #[test]
    fn test_search_error_display() {
        assert_eq!(
            SearchError::Timeout.to_string(),
            "timeout when doing the petition."
        );
        assert_eq!(
            SearchError::NotEnoughSimilarity.to_string(),
            "not found any result above the minimum similarity rate."
        );
        assert_eq!(SearchError::NotFound.to_string(), "not found any result.");
    }

    /// Test the search error debug implementation.
    #[test]
    fn test_search_error_debug() {
        assert_eq!(format!("{:?}", SearchError::Timeout), "Timeout");
        assert_eq!(
            format!("{:?}", SearchError::NotEnoughSimilarity),
            "NotEnoughSimilarity"
        );
        assert_eq!(format!("{:?}", SearchError::NotFound), "NotFound");
    }
}
