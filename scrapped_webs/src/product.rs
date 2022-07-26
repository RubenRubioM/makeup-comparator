//! Product struct declaration and implementations.

use std::fmt::Display;

/// Defines a tone.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Tone {
    /// Name of the tone.
    name: String,
    /// The standard price.
    price_standard: f32,
    ///The price in case it is on sale.
    price_sales: Option<f32>,
}

impl Tone {
    pub fn new(name: String, price_standard: f32, price_sales: Option<f32>) -> Self {
        Self {
            name,
            price_standard,
            price_sales,
        }
    }

    /// Returns the name of the product.
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Returns the standard price.
    pub fn price_standard(&self) -> f32 {
        self.price_standard
    }
    /// Returns the price if it is on sale.
    pub fn price_sales(&self) -> Option<f32> {
        self.price_sales
    }
    /// Sets the name of the product.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Sets the standard price.
    pub fn set_price_standard(&mut self, price_standard: f32) {
        self.price_standard = price_standard;
    }
    /// Sets the price if it is on sale.
    pub fn set_price_sales(&mut self, price_sales: Option<f32>) {
        self.price_sales = price_sales;
    }
}

/// Defines a product we can obtain web scraping the website
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Product {
    /// The product name.
    name: String,
    /// The link to the product.
    link: String,
    /// The standard price.
    price_standard: f32,
    /// The price in case it is on sale.
    price_sales: Option<f32>,
    /// The list of tones for this product.
    tones: Option<Vec<Tone>>,
    /// The ratings between 0-5.
    rating: Option<f32>,
    /// Similarity between the product name to search and the one found.
    similarity: f32,
}

impl Product {
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
        name: String,
        link: String,
        price_standard: f32,
        price_sales: Option<f32>,
        tones: Option<Vec<Tone>>,
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
    pub fn name(&self) -> String {
        self.name.clone()
    }
    /// Returns the link to the product.
    pub fn link(&self) -> String {
        self.link.clone()
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
    /// Sets the name of the product.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Sets the link to the product.
    pub fn set_link(&mut self, link: String) {
        self.link = link;
    }
    /// Sets the standard price.
    pub fn set_price_standard(&mut self, price_standard: f32) {
        self.price_standard = price_standard;
    }
    /// Sets the price if it is on sale.
    pub fn set_price_sales(&mut self, price_sales: Option<f32>) {
        self.price_sales = price_sales
    }
    /// Sets the tones of the product.
    pub fn set_tones(&mut self, tones: Option<Vec<Tone>>) {
        self.tones = tones;
    }
    /// Sets the rating.
    pub fn set_rating(&mut self, rating: Option<f32>) {
        self.rating = rating;
    }
    /// Sets the similarity.
    pub fn set_similarity(&mut self, similarity: f32) {
        self.similarity = similarity;
    }
}

// TODO
impl Display for Product {
    #[allow(unused_variables)] // To prevent the warning
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
