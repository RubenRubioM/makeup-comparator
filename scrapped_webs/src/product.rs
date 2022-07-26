//! Product struct declaration and implementations.

use std::fmt::Display;

/// Defines a tone.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Tone<'a> {
    /// Name of the tone.
    name: &'a str,
    /// The standard price.
    price_standard: f32,
    ///The price in case it is on sale.
    price_sales: Option<f32>,
}

impl<'a> Tone<'a> {
    pub fn new(name: &'a str, price_standard: f32, price_sales: Option<f32>) -> Self {
        Self {
            name,
            price_standard,
            price_sales,
        }
    }

    /// Returns the name of the product.
    pub fn name(&self) -> &'a str {
        self.name
    }
    /// Returns the standard price.
    pub fn price_standard(&self) -> f32 {
        self.price_standard
    }
    /// Returns the price if it is on sale.
    pub fn price_sales(&self) -> Option<f32> {
        self.price_sales
    }
}

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
    tones: Option<Vec<Tone<'a>>>,
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
        tones: Option<Vec<Tone<'a>>>,
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
    /// Returns the tones of the product.
    pub fn tones(&self) -> Option<&Vec<Tone>> {
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
