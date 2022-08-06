//! Trait that defines the scrappable trait
use crate::product::Product;

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
}
