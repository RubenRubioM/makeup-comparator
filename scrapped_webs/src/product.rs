//! Product struct declaration and implementations.

use std::fmt::Display;

/// Defines a product we can obtain web scraping the website
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Product<'a> {
    /// The product name.
    name: &'a str,
    /// The link to the product.
    link: &'a str,
    /// The standard price.
    price_standard: f32,
    /// The price in case it is on sale.
    price_sales: Option<f32>,
    /// The list of tones for this product.
    tones: Option<Vec<&'a str>>,
    /// The ratings between 0-5.
    rating: Option<f32>,
    /// Similarity between the product name to search and the one found.
    similarity: f32,
}

impl<'a> Product<'a> {
    /// Creates a new Product.
    /// # Arguments
    /// name: The name of the product.
    /// link: The url to the product.
    /// price_standard: The price of the product.
    /// price_sales: The price of the product if it is on sale.
    /// tones: Varieties of the product.
    /// rating: The rating of the product.
    /// similarity: Similarity with the product tried to find.
    /// # Returns
    /// Self: A Product.
    pub fn new(
        name: &'a str,
        link: &'a str,
        price_standard: f32,
        price_sales: Option<f32>,
        tones: Option<Vec<&'a str>>,
        rating: Option<f32>,
        similarity: f32,
    ) -> Self {
        Self {
            name,
            link,
            price_standard,
            price_sales,
            tones,
            rating,
            similarity,
        }
    }
    /// Returns the name of the product.
    pub fn name(&self) -> &'a str {
        self.name
    }
    /// Returns the link to the product.
    pub fn link(&self) -> &'a str {
        self.link
    }
    /// Returns the standard price.
    pub fn price_standard(&self) -> f32 {
        self.price_standard
    }
    /// Returns the price if it is on sale.
    pub fn price_sales(&self) -> Option<f32> {
        self.price_sales
    }
    /// Returns (discount_value, percentage_discount) if we have price_sales.
    ///
    /// # Example
    ///
    /// price_standard = 30;
    /// price_sales = 15;
    /// (15.0, 50)
    pub fn discount(&self) -> Option<(f32, u8)> {
        if let Some(price_sales) = self.price_sales {
            let discount: u8 = (price_sales / self.price_standard).round() as u8 * 100;
            let discount_value: f32 = self.price_standard - price_sales;
            return Some((discount_value, discount));
        }
        None
    }
    /// Returns the tones of the product.
    pub fn tones(&self) -> Option<&Vec<&'a str>> {
        self.tones.as_ref()
    }
    /// Returns the rating.
    pub fn rating(&self) -> Option<f32> {
        self.rating
    }
    /// Returns the similarity.
    pub fn similarity(&self) -> f32 {
        self.similarity
    }
}

// TODO
impl<'a> Display for Product<'a> {
    #[allow(unused_variables)] // To prevent the warning
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}