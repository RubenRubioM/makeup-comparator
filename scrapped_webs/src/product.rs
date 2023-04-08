//! Product struct declaration and implementations.

use ansi_term;
use std::fmt::Display;

use crate::helper::utilities;

/// Defines a tone.
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Tone {
    /// Name of the tone.
    pub name: Option<String>,
    /// The standard price.
    pub price_standard: Option<f32>,
    /// The price in case it is on sale.
    pub price_sales: Option<f32>,
    /// Available flag.
    pub available: bool,
    /// Possible url if it is not directly in the same webpage.
    pub url: Option<String>,
    /// Possible rating.
    pub rating: Option<f32>,
}

impl Tone {
    pub fn new(
        name: Option<String>,
        price_standard: Option<f32>,
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
    /// if available, on sale and rating = ✔️ Name -  ̶9̶.̶9̶9̶  4.99(50%) - 9.5⭐
    /// if not available, not on sale and no rating = ❌ Name - 9.99
    pub fn terminal_format(&self) -> String {
        let mut out: String = String::from("    - ");
        match self.available {
            true => out.push_str("✔️   "),
            false => out.push_str("❌   "),
        }
        out.push_str(format!("{} - ", self.name.as_ref().unwrap()).as_str());
        match self.price_sales {
            Some(price_sales) => {
                let strikedthrought_price = ansi_term::Style::new()
                    .strikethrough()
                    .paint(self.price_standard.unwrap().to_string())
                    .to_string();
                out.push_str(
                    format!(
                        "{}€ {}€({}%)",
                        strikedthrought_price,
                        price_sales,
                        utilities::discount(self.price_standard.unwrap(), self.price_sales)
                            .unwrap()
                            .1
                    )
                    .as_str(),
                )
            }
            None => out.push_str(format!("{}€", self.price_standard.unwrap()).as_str()),
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
            self.price_standard.unwrap()
        }
    }
}

impl Display for Tone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out: String = String::new();
        out.push_str(format!("Name: {}", self.name.as_ref().unwrap()).as_str());
        out.push_str(format!("\nPrice: {}", self.price_standard.unwrap()).as_str());
        if let Some(price_sales) = self.price_sales {
            out.push_str(format!("\nPrice on sale: {price_sales}").as_str());
        }

        writeln!(f, "{out}")
    }
}

/// Defines a product we can obtain web scraping the website
#[derive(Clone, PartialEq, PartialOrd, Debug, Default)]
pub struct Product {
    /// The product name.
    pub name: String,
    /// The brand name.
    pub brand: Option<String>,
    /// The link to the product.
    pub link: String,
    /// The standard price.
    pub price_standard: Option<f32>,
    /// The price in case it is on sale.
    pub price_sales: Option<f32>,
    /// The ratings between 0-5.
    pub rating: Option<f32>,
    /// Similarity between the product name to search and the one found.
    pub similarity: f32,
    /// Available of the product.
    pub available: bool,
    /// The list of tones for this product.
    pub tones: Option<Vec<Tone>>,
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
        brand: Option<String>,
        link: String,
        price_standard: Option<f32>,
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
    ///  if has tones: 95%. Labial Rare Beauty - 10.99-15.99 - 9.5⭐: www.test.com
    ///  if doesn't have tones: 95% - Labial Rare Beauty - 10.99 - 9.5⭐: www.test.com
    pub fn terminal_format(&self) -> String {
        let mut out: String = String::new();
        out.push_str(format!("- {}. ", self.similarity_formatted()).as_str());
        out.push_str(format!("{} - {} - ", self.name, self.brand.as_ref().unwrap()).as_str());

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
                out.push_str(format!("{lowest_price}€-{highest_price}€").as_str());
            }
            None => match self.price_sales {
                Some(price_sales) => {
                    let strikedthrought_price = ansi_term::Style::new()
                        .strikethrough()
                        .paint(self.price_standard.unwrap().to_string())
                        .to_string();
                    out.push_str(
                        format!(
                            "{}€ {}€({}%)",
                            strikedthrought_price,
                            price_sales,
                            utilities::discount(self.price_standard.unwrap(), self.price_sales)
                                .unwrap()
                                .1
                        )
                        .as_str(),
                    )
                }
                None => out.push_str(format!("{}€", self.price_standard.unwrap()).as_str()),
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
        out.push_str(format!("Name: {}", self.name).as_str());
        out.push_str(format!("\nBrand: {}", self.brand.as_ref().unwrap()).as_str());
        out.push_str(format!("\nLink: {}", self.link).as_str());
        out.push_str(format!("\nPrice: {}", self.price_standard.unwrap()).as_str());
        if let Some(price_sales) = self.price_sales {
            out.push_str(format!("\nPrice on sale: {price_sales}").as_str());
        }
        if let Some(rating) = self.rating {
            out.push_str(format!("\nRating: {rating}").as_str());
        }
        out.push_str(format!("\nSimilarity: {}", self.similarity).as_str());
        out.push_str(format!("\nAvailable: {}", self.available).as_str());
        if let Some(tones) = self.tones.as_ref() {
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
        let name: Option<String> = Some(String::from("Tone1"));
        let price_standard: Option<f32> = Some(50.0);
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
        assert_eq!(*tone.name.unwrap(), name.unwrap());
        assert_eq!(tone.price_standard.unwrap(), price_standard.unwrap());
        assert_eq!(tone.price_sales.unwrap(), price_sales.unwrap());
        assert_eq!(tone.available, available);
        assert_eq!(tone.url.unwrap(), url.unwrap());
        assert_eq!(tone.rating.unwrap(), rating.unwrap());

        // Setters
        let set_name = String::from("Tone2");
        let set_price_standard: f32 = 100.0;
        let set_price_sales: Option<f32> = Some(50.0);
        let set_available: bool = false;
        let set_url: Option<String> = Some(String::from("www.tone2.es"));
        let set_rating: Option<f32> = Some(4.0);

        tone.name = Some(set_name.clone());
        tone.price_standard = Some(set_price_standard);
        tone.price_sales = set_price_sales.clone();
        tone.available = set_available;
        tone.url = set_url.clone();
        tone.rating = set_rating.clone();

        println!("Testing Debug trait implementation for Tone: {:?}", tone);
        println!("Testing Display trait implementation for Tone: {}", tone);
    }

    /// Tests the Product::new function.
    #[test]
    fn product_instantiation_getters_and_setters() {
        let name: String = String::from("Test");
        let brand: Option<String> = Some(String::from("Test Brand"));
        let link: String = String::from("http://test.es");
        let tone_name: Option<String> = Some(String::from("Tone 1"));
        let price_standard: Option<f32> = Some(50.0);
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
        assert_eq!(*product.name, name);
        assert_eq!(*product.brand.unwrap(), brand.unwrap());
        assert_eq!(*product.link, link);
        assert_eq!(product.price_standard.unwrap(), price_standard.unwrap());
        assert_eq!(product.price_sales.unwrap(), price_sales.unwrap());
        assert_eq!(product.rating.unwrap(), rating.unwrap());
        assert_eq!(product.similarity, similarity);
        assert_eq!(product.available, available);

        assert_eq!(
            *product
                .tones
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .name
                .as_ref()
                .unwrap(),
            tone_name.unwrap()
        );
        assert_eq!(
            product
                .tones
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .price_standard
                .unwrap(),
            price_standard.unwrap()
        );
        assert_eq!(
            product
                .tones
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .price_sales
                .unwrap(),
            price_sales.unwrap()
        );
        assert_eq!(
            product.tones.as_ref().unwrap().first().unwrap().available,
            available
        );
        assert_eq!(
            product
                .tones
                .as_ref()
                .unwrap()
                .first()
                .unwrap()
                .url
                .as_deref()
                .unwrap(),
            url.unwrap()
        );
        assert_eq!(
            product.tones.unwrap().first().unwrap().rating.unwrap(),
            tone_rating.unwrap()
        );

        // Setters
        let set_name: String = String::from("Test 2");
        let set_brand: Option<String> = Some(String::from("Test Brand 2"));
        let set_link: String = String::from("http://test2.es");
        let set_tone_name: Option<String> = Some(String::from("Tone 2"));
        let set_price_standard: Option<f32> = Some(100.0);
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

        product.name = set_name.clone();
        product.brand = set_brand.clone();
        product.link = set_link.clone();
        product.price_standard = set_price_standard;
        product.price_sales = set_price_sales.clone();
        product.tones = set_tones.clone();
        product.rating = set_rating.clone();
        product.similarity = set_similarity;
        product.available = set_available;

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
        let price_standard: Option<f32> = Some(10.0);
        let price_sales: Option<f32> = Some(5.0);
        let tone_on_sale: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard,
            price_sales: price_sales,
            available: true,
            url: None,
            rating: Some(9.5),
        };
        assert_eq!(tone_on_sale.price(), price_sales.unwrap());

        // Tone without price on sale.
        let tone: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard,
            price_sales: None,
            available: true,
            url: None,
            rating: Some(9.5),
        };
        assert_eq!(tone.price(), price_standard.unwrap());
    }

    /// Tests the function Product::terminal_format without tones and on sale.
    #[test]
    fn product_format_terminal_without_tones() {
        let product: Product = Product {
            name: String::from("Product 1"),
            brand: Some(String::from("Brand")),
            link: String::from("http://www.test.com"),
            price_standard: Some(10.0),
            price_sales: None,
            rating: Some(9.5),
            similarity: 0.9,
            available: true,
            tones: None,
        };
        product.terminal_format();

        let product_on_sale: Product = Product {
            name: String::from("Product 1"),
            brand: Some(String::from("Brand")),
            link: String::from("http://www.test.com"),
            price_standard: Some(10.0),
            price_sales: Some(5.0),
            rating: Some(9.5),
            similarity: 0.9,
            available: true,
            tones: None,
        };
        product_on_sale.terminal_format();
        // assert_eq!(product.terminal_format(), "90%. Product 1 Brand - 10€ 5€(50%) - 9.5⭐: http://www.test.com");
    }

    /// Tests the function Product::terminal_format with tones.
    #[test]
    fn product_format_terminal_with_tones() {
        let tone: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard: Some(50.99),
            price_sales: None,
            available: true,
            url: None,
            rating: None,
        };
        let tone_on_sale: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard: Some(10.0),
            price_sales: Some(5.0),
            available: true,
            url: None,
            rating: None,
        };

        let product: Product = Product {
            name: String::from("Product 1"),
            brand: Some(String::from("Brand")),
            link: String::from("http://www.test.com"),
            price_standard: Some(10.0),
            price_sales: Some(5.0),
            rating: Some(9.5),
            similarity: 0.95421,
            available: true,
            tones: Some(vec![tone, tone_on_sale]),
        };
        assert_eq!(
            product.terminal_format(),
            "- 95.42%. Product 1 - Brand - 5€-50.99€ - 9.5⭐: http://www.test.com"
        );
    }

    /// Tests the function Tone::terminal_format with a tone available, on sale and with rating
    #[test]
    fn tone_format_terminal_available_on_sale_with_rating() {
        let tone: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard: Some(10.0),
            price_sales: Some(5.0),
            available: true,
            url: None,
            rating: Some(9.5),
        };
        tone.terminal_format();
        // assert_eq!(output, "✔️   Tone 1 -  ̶10€ 5€(50%) - 9.5⭐"); Can not test strikethrough text
    }

    /// Tests the function Tone::terminal_format with a tone unavailable and without rating
    #[test]
    fn tone_format_terminal_unavailable_without_rating() {
        let tone: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard: Some(10.0),
            price_sales: None,
            available: false,
            url: None,
            rating: None,
        };
        assert_eq!(tone.terminal_format(), "    - ❌   Tone 1 - 10€");
    }

    /// Tests the function Tone::terminal_format with a tone unavailable and with rating
    #[test]
    fn tone_format_terminal_unavailable_with_rating() {
        let tone: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard: Some(10.0),
            price_sales: None,
            available: false,
            url: None,
            rating: Some(9.5),
        };
        assert_eq!(tone.terminal_format(), "    - ❌   Tone 1 - 10€ - 9.5⭐");
    }

    /// Tests the function Tone::terminal_format with a tone unavailable, on sale and without rating
    #[test]
    fn tone_format_terminal_unavailable_on_sale_without_rating() {
        let tone: Tone = Tone {
            name: Some(String::from("Tone 1")),
            price_standard: Some(10.0),
            price_sales: Some(5.0),
            available: false,
            url: None,
            rating: None,
        };
        tone.terminal_format();
        // assert_eq!(output, "❌   Tone 1 -  ̶10€ 5€(50%)"); Can not test strikethrough text
    }
}
