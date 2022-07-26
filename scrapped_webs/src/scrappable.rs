//! Trait that defines the scrappable trait
use crate::product::Product;

/// Enumeration of possible error when trying to search a product.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum SearchError {
    /// Timeout in the request.
    Timeout,
    /// Found products but without enought similarity with the one provided.
    NotEnoughtSimilarity,
    /// Not found any result.
    NotFound,
}

impl std::fmt::Display for SearchError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SearchError::Timeout => write!(f, "Timeout when doing the petition"),
            SearchError::NotEnoughtSimilarity => {
                write!(f, "Not found any result above the minimum similarity rate")
            }
            SearchError::NotFound => write!(f, "Not found any result"),
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
    fn look_for_products<'a>(name: &str) -> Result<Vec<Product>, SearchError>;
}
