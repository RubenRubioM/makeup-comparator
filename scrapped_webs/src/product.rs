//! Product struct declaration and implementations.

use ansi_term;
use std::fmt::Display;

use crate::helper;

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

    /// Formats the Tone object to be pretty printed in terminal.
    /// # Example
    /// if available, on sale and rating = ✔️ Name -  ̶9̶.̶9̶9̶  4.99(50%) - 9.5
    /// if not available, not on sale and no rating = ❌ Name - 9.99
    pub fn terminal_format(&self) -> String {
        let mut out: String = String::new();
        match self.available {
            true => out.push_str("✔️ "),
            false => out.push_str("❌ "),
        }
        out.push_str(format!("{} - ", self.name).as_str());
        match self.price_sales {
            Some(price_sales) => {
                let strikedthrought_price = ansi_term::Style::new()
                    .strikethrough()
                    .paint(self.price_standard.to_string())
                    .to_string();
                out.push_str(
                    format!(
                        "{} {}({}%)",
                        strikedthrought_price,
                        price_sales,
                        helper::discount(self.price_standard, self.price_sales)
                            .unwrap()
                            .1
                    )
                    .as_str(),
                )
            }
            None => out.push_str(format!("{}", self.price_standard).as_str()),
        }
        if let Some(rating) = self.rating {
            out.push_str(format!(" - {rating}⭐").as_str())
        }
        out
    }

    /// Returns the actual price, doesn't matter if on sale or not
    pub fn price(&self) -> f32 {
        if self.price_sales.is_some() {
            self.price_sales.unwrap()
        } else {
            self.price_standard
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
            out.push_str(format!("\nPrice on sale: {price_sales}").as_str());
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

    /// Formats the Tone object to be pretty printed in terminal.
    /// # Example
    ///  if has tones: 95%. Labial Rare Beauty - 10.99-15.99 - 9.5: www.test.com
    ///  if doesn't have tones: 95% - Labial Rare Beauty - 10.99 - 9.5: www.test.com
    pub fn terminal_format(&self) -> String {
        let mut out: String = String::new();
        out.push_str(format!("{}. ", self.similarity_formatted()).as_str());
        out.push_str(format!("{} {} - ", self.name, self.brand).as_str());

        match self.tones.as_ref() {
            // If we have tones, look for the lowest and highest price
            Some(tones) => {
                let mut lowest_price: f32 = f32::MAX;
                let mut highest_price: f32 = f32::MIN;
                for tone in tones {
                    let price = tone.price();
                    if price > highest_price {
                        highest_price = price;
                    }
                    if price < lowest_price {
                        lowest_price = price;
                    }
                }
                out.push_str(format!("{lowest_price}-{highest_price}").as_str());
            }
            None => match self.price_sales {
                Some(price_sales) => {
                    let strikedthrought_price = ansi_term::Style::new()
                        .strikethrough()
                        .paint(self.price_standard.to_string())
                        .to_string();
                    out.push_str(
                        format!(
                            "{} {}({}%)",
                            strikedthrought_price,
                            price_sales,
                            helper::discount(self.price_standard, self.price_sales)
                                .unwrap()
                                .1
                        )
                        .as_str(),
                    )
                }
                None => out.push_str(format!("{}", self.price_standard).as_str()),
            },
        }

        if let Some(rating) = self.rating {
            out.push_str(format!(" - {rating}⭐").as_str());
        }
        out.push_str(format!(": {}", self.link).as_str());
        out
    }

    /// Returns the similarity rounded and formatted
    /// # Example
    /// .621242 = 62.12%
    fn similarity_formatted(&self) -> String {
        format!("{:.2}%", self.similarity * 100.0)
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
            out.push_str(format!("\nPrice on sale: {}", price_sales).as_str());
        }
        if let Some(rating) = self.rating() {
            out.push_str(format!("\nRating: {rating}").as_str());
        }
        out.push_str(format!("\nSimilarity: {}", self.similarity()).as_str());
        out.push_str(format!("\nAvailable: {}", self.available()).as_str());
        if let Some(tones) = self.tones() {
            out.push_str(format!("\nTones: {tones:#?}").as_str());
        }

        writeln!(f, "{out}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the Tone::new function.
    #[test]
    fn tone_instantiation_getters_and_setters() {
        let name: String = String::from("Tone1");
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let available: bool = true;
        let url: Option<String> = Some(String::from("www.tone.es"));
        let rating: Option<f32> = Some(5.0);
        let mut tone: Tone = Tone::new(
            name.clone(),
            price_standard,
            price_sales,
            available,
            url.clone(),
            rating.clone(),
        );

        // Getters
        assert_eq!(*tone.name(), name);
        assert_eq!(tone.price_standard(), price_standard);
        assert_eq!(tone.price_sales().unwrap(), price_sales.unwrap());
        assert_eq!(tone.available(), available);
        assert_eq!(tone.url(), url);
        assert_eq!(tone.rating(), rating);

        // Setters
        let set_name = String::from("Tone2");
        let set_price_standard: f32 = 100.0;
        let set_price_sales: Option<f32> = Some(50.0);
        let set_available: bool = false;
        let set_url: Option<String> = Some(String::from("www.tone2.es"));
        let set_rating: Option<f32> = Some(4.0);

        tone.set_name(set_name.clone());
        tone.set_price_standard(set_price_standard);
        tone.set_price_sales(set_price_sales.clone());
        tone.set_available(set_available);
        tone.set_url(set_url.clone());
        tone.set_rating(set_rating.clone());

        println!("Testing Debug trait implementation for Tone: {:?}", tone);
        println!("Testing Display trait implementation for Tone: {}", tone);
    }

    /// Tests the Product::new function.
    #[test]
    fn product_instantiation_getters_and_setters() {
        let name: String = String::from("Test");
        let brand: String = String::from("Test Brand");
        let link: String = String::from("http://test.es");
        let tone_name: String = String::from("Tone 1");
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let available: bool = true;
        let url: Option<String> = Some(String::from("www.tone.es"));
        let tone_rating: Option<f32> = Some(5.0);
        let tones: Option<Vec<Tone>> = Some(vec![Tone::new(
            tone_name.clone(),
            price_standard,
            price_sales,
            available,
            url.clone(),
            tone_rating.clone(),
        )]);
        let rating: Option<f32> = Some(4.5);
        let similarity: f32 = 0.86;
        let mut product: Product = Product::new(
            name.clone(),
            brand.clone(),
            link.clone(),
            price_standard,
            price_sales,
            tones,
            rating,
            similarity,
            available,
        );

        // Getters
        assert_eq!(*product.name(), name);
        assert_eq!(*product.brand(), brand);
        assert_eq!(*product.link(), link);
        assert_eq!(product.price_standard(), price_standard);
        assert_eq!(product.price_sales().unwrap(), price_sales.unwrap());
        assert_eq!(product.rating().unwrap(), rating.unwrap());
        assert_eq!(product.similarity(), similarity);
        assert_eq!(product.available(), available);

        assert_eq!(*product.tones().unwrap().first().unwrap().name(), tone_name);
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_standard(),
            price_standard
        );
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_sales(),
            price_sales
        );
        assert_eq!(
            product.tones().unwrap().first().unwrap().available(),
            available
        );
        assert_eq!(product.tones().unwrap().first().unwrap().url(), url);
        assert_eq!(
            product.tones().unwrap().first().unwrap().rating(),
            tone_rating
        );

        // Setters
        let set_name: String = String::from("Test 2");
        let set_brand: String = String::from("Test Brand 2");
        let set_link: String = String::from("http://test2.es");
        let set_tone_name: String = String::from("Tone 2");
        let set_price_standard: f32 = 100.0;
        let set_price_sales: Option<f32> = Some(50.0);
        let set_available: bool = false;
        let set_url: Option<String> = Some(String::from("www.tone2.es"));
        let set_tone_rating: Option<f32> = Some(4.0);
        let set_tones: Option<Vec<Tone>> = Some(vec![Tone::new(
            set_tone_name.clone(),
            set_price_standard,
            set_price_sales,
            set_available,
            set_url.clone(),
            set_tone_rating.clone(),
        )]);
        let set_rating: Option<f32> = Some(4.0);
        let set_similarity: f32 = 0.75;

        product.set_name(set_name.clone());
        product.set_brand(set_brand.clone());
        product.set_link(set_link.clone());
        product.set_price_standard(set_price_standard);
        product.set_price_sales(set_price_sales.clone());
        product.set_tones(set_tones.clone());
        product.set_rating(set_rating.clone());
        product.set_similarity(set_similarity);
        product.set_available(set_available);

        println!(
            "Testing Debug trait implementation for Product: {:?}",
            product
        );
        println!(
            "Testing Display trait implementation for Product: {}",
            product
        );
    }

    /// Tests the price function for the tone
    #[test]
    fn price_all_paths() {
        // Tone with price on sale.
        let price_standard: f32 = 10.0;
        let price_sales: f32 = 5.0;
        let tone_on_sale: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard,
            price_sales: Some(price_sales),
            available: true,
            url: None,
            rating: Some(9.5),
        };
        assert_eq!(tone_on_sale.price(), price_sales);

        // Tone without price on sale.
        let tone: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard,
            price_sales: None,
            available: true,
            url: None,
            rating: Some(9.5),
        };
        assert_eq!(tone.price(), price_standard);
    }

    /// Tests the function Product::terminal_format without tones and on sale.
    #[test]
    fn product_format_terminal_without_tones() {
        let product: Product = Product {
            name: String::from("Product 1"),
            brand: String::from("Brand"),
            link: String::from("http://www.test.com"),
            price_standard: 10.0,
            price_sales: None,
            rating: Some(9.5),
            similarity: 0.9,
            available: true,
            tones: None,
        };
        product.terminal_format();

        let product_on_sale: Product = Product {
            name: String::from("Product 1"),
            brand: String::from("Brand"),
            link: String::from("http://www.test.com"),
            price_standard: 10.0,
            price_sales: Some(5.0),
            rating: Some(9.5),
            similarity: 0.9,
            available: true,
            tones: None,
        };
        product_on_sale.terminal_format();
        // assert_eq!(product.terminal_format(), "90%. Product 1 Brand - 10 5(50%) - 9.5⭐: http://www.test.com");
    }

    /// Tests the function Product::terminal_format with tones.
    #[test]
    fn product_format_terminal_with_tones() {
        let tone: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard: 50.99,
            price_sales: None,
            available: true,
            url: None,
            rating: None,
        };
        let tone_on_sale: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard: 10.0,
            price_sales: Some(5.0),
            available: true,
            url: None,
            rating: None,
        };

        let product: Product = Product {
            name: String::from("Product 1"),
            brand: String::from("Brand"),
            link: String::from("http://www.test.com"),
            price_standard: 10.0,
            price_sales: Some(5.0),
            rating: Some(9.5),
            similarity: 0.95421,
            available: true,
            tones: Some(vec![tone, tone_on_sale]),
        };
        assert_eq!(
            product.terminal_format(),
            "95.42%. Product 1 Brand - 5-50.99 - 9.5⭐: http://www.test.com"
        );
    }

    /// Tests the function Tone::terminal_format with a tone available, on sale and with rating
    #[test]
    fn tone_format_terminal_available_on_sale_with_rating() {
        let tone: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard: 10.0,
            price_sales: Some(5.0),
            available: true,
            url: None,
            rating: Some(9.5),
        };
        tone.terminal_format();
        // assert_eq!(output, "✔️ Tone 1 -  ̶10 5(50%) - 9.5⭐"); Can not test strikethrough text
    }

    /// Tests the function Tone::terminal_format with a tone unavailable and without rating
    #[test]
    fn tone_format_terminal_unavailable_without_rating() {
        let tone: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard: 10.0,
            price_sales: None,
            available: false,
            url: None,
            rating: None,
        };
        assert_eq!(tone.terminal_format(), "❌ Tone 1 - 10");
    }

    /// Tests the function Tone::terminal_format with a tone unavailable and with rating
    #[test]
    fn tone_format_terminal_unavailable_with_rating() {
        let tone: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard: 10.0,
            price_sales: None,
            available: false,
            url: None,
            rating: Some(9.5),
        };
        assert_eq!(tone.terminal_format(), "❌ Tone 1 - 10 - 9.5⭐");
    }

    /// Tests the function Tone::terminal_format with a tone unavailable, on sale and without rating
    #[test]
    fn tone_format_terminal_unavailable_on_sale_without_rating() {
        let tone: Tone = Tone {
            name: String::from("Tone 1"),
            price_standard: 10.0,
            price_sales: Some(5.0),
            available: false,
            url: None,
            rating: None,
        };
        tone.terminal_format();
        // assert_eq!(output, "❌ Tone 1 -  ̶10 5(50%)"); Can not test strikethrough text
    }
}
