//! Trait that defines the scrappable trait
use scraper::{ElementRef, Html};

use crate::product::{Product, Tone};

/// Enumeration of possible error when trying to search a product.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SearchError {
    /// Timeout in the request.
    Timeout,
    /// Found products but without enough similarity with the one provided.
    NotEnoughSimilarity,
    /// Not found any result.
    NotFound,
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Timeout => write!(f, "timeout when doing the petition"),
            SearchError::NotEnoughSimilarity => {
                write!(f, "not found any result above the minimum similarity rate")
            }
            SearchError::NotFound => write!(f, "not found any result"),
        }
    }
}

pub trait Scrappable {
    /// Try to find the product in the website.
    ///
    /// # Arguments
    /// name - The name of the product to find.
    /// # Returns
    /// Product - A vector with the similar products that matches the name.
    /// Box<dyn Error> - If couldn't find the product.
    fn look_for_products(&self, name: String) -> Result<Vec<Product>, SearchError>;

    /// Returns the url of the products found.
    /// # Arguments
    /// document - The search page HTML document with some or none products found.
    /// name - The name provided by the user to find.
    /// # Returns
    /// Ok - Vector with the urls found in the search page.
    /// Err - Search error.
    fn search_results_urls(&self, document: &Html, name: &str) -> Result<Vec<String>, SearchError>;

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
