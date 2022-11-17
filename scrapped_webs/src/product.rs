//! Product struct declaration and implementations.

use std::fmt::Display;

/// Defines a tone.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Tone {
    /// Name of the tone.
    name: String,
    /// The standard price.
    price_standard: f32,
    /// The price in case it is on sale.
    price_sales: Option<f32>,
    /// Available flag.
    available: bool,
    /// Possible url if it is not directly in the same webpage.
    url: Option<String>,
    /// Possible rating.
    rating: Option<f32>,
}

impl Tone {
    pub fn new(
        name: String,
        price_standard: f32,
        price_sales: Option<f32>,
        available: bool,
        url: Option<String>,
        rating: Option<f32>,
    ) -> Self {
        Self {
            name,
            price_standard,
            price_sales,
            available,
            url,
            rating,
        }
    }

    /// Returns the name of the product.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Returns the standard price.
    pub fn price_standard(&self) -> f32 {
        self.price_standard
    }
    /// Returns the price if it is on sale.
    pub fn price_sales(&self) -> Option<f32> {
        self.price_sales
    }
    /// Returns the available.
    pub fn available(&self) -> bool {
        self.available
    }
    /// Returns the url.
    pub fn url(&self) -> Option<String> {
        self.url.clone()
    }
    /// Returns the rating.
    pub fn rating(&self) -> Option<f32> {
        self.rating
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
    /// Sets if the product its available.
    pub fn set_available(&mut self, available: bool) {
        self.available = available;
    }
    /// Sets the URL.
    pub fn set_url(&mut self, url: Option<String>) {
        self.url = url;
    }
    /// Sets the rating.
    pub fn set_rating(&mut self, rating: Option<f32>) {
        self.rating = rating;
    }
}

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        out.push_str(format!("Name: {}", self.name()).as_str());
        out.push_str(format!("\nPrice: {}", self.price_standard()).as_str());
        if let Some(price_sales) = self.price_sales {
            out.push_str(format!("\nPrice on sale: {}", price_sales).as_str())
        }

        writeln!(f, "{}", out)
    }
}

/// Defines a product we can obtain web scraping the website
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Product {
    /// The product name.
    name: String,
    /// The brand name.
    brand: String,
    /// The link to the product.
    link: String,
    /// The standard price.
    price_standard: f32,
    /// The price in case it is on sale.
    price_sales: Option<f32>,
    /// The ratings between 0-5.
    rating: Option<f32>,
    /// Similarity between the product name to search and the one found.
    similarity: f32,
    /// Available of the product.
    available: bool,
    /// The list of tones for this product.
    tones: Option<Vec<Tone>>,
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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        brand: String,
        link: String,
        price_standard: f32,
        price_sales: Option<f32>,
        tones: Option<Vec<Tone>>,
        rating: Option<f32>,
        similarity: f32,
        available: bool,
    ) -> Self {
        Self {
            name,
            brand,
            link,
            price_standard,
            price_sales,
            tones,
            rating,
            similarity,
            available,
        }
    }
    /// Returns the name of the product.
    pub fn name(&self) -> &String {
        &self.name
    }
    /// Returns the brand of the product.
    pub fn brand(&self) -> &String {
        &self.brand
    }
    /// Returns the link to the product.
    pub fn link(&self) -> &String {
        &self.link
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
    /// Returns the availability.
    pub fn available(&self) -> bool {
        self.available
    }
    /// Sets the name of the product.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    /// Sets the brand of the product.
    pub fn set_brand(&mut self, brand: String) {
        self.brand = brand;
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
    /// Sets the availability.
    pub fn set_available(&mut self, set_available: bool) {
        self.available = set_available;
    }
    /// Adds a new Tone.
    pub fn add_tone(&mut self, tone: Tone) {
        if self.tones.is_none() {
            self.tones = Some(Vec::<Tone>::new());
        }
        self.tones.as_mut().unwrap().push(tone);
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        out.push_str(format!("Name: {}", self.name()).as_str());
        out.push_str(format!("\nBrand: {}", self.brand()).as_str());
        out.push_str(format!("\nLink: {}", self.link()).as_str());
        out.push_str(format!("\nPrice: {}", self.price_standard()).as_str());
        if let Some(price_sales) = self.price_sales() {
            out.push_str(format!("\nPrice on sale: {}", price_sales).as_str())
        }
        if let Some(rating) = self.rating() {
            out.push_str(format!("\nRating: {}", rating).as_str()),
        }
        out.push_str(format!("\nSimilarity: {}", self.similarity()).as_str());
        out.push_str(format!("\nAvailable: {}", self.available()).as_str());
        if let Some(tones) = self.tones() {
            out.push_str(format!("\nTones: {:#?}", tones).as_str())
        }

        writeln!(f, "{}", out)
    }
}
