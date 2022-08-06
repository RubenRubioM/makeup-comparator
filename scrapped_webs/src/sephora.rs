//! This file encapsulate the different sephoras in the world

use std::thread;
use std::thread::JoinHandle;

use crate::configuration::Configuration;
use crate::helper;
use crate::{
    product::Product,
    product::Tone,
    scrappable::{Scrappable, SearchError},
};
use scraper::ElementRef;
use scraper::Html;

/// Module for sephora.es
pub mod spain {
    use super::*;
    // Webpage url.
    const URL: &str = "https://www.sephora.es/";
    // Suffix for searching in the website.
    const SEARCH_SUFFIX: &str = "buscar?q=";
    // Maximum rating for SephoraSpain.
    // const MAX_RATING: f32 = 5.0;

    /// Structure that define functionality for SephoraSpain.
    pub struct SephoraSpain<'a> {
        pub config: &'a Configuration,
    }

    /// Implementation for SephoraSpain
    impl<'a> SephoraSpain<'a> {
        /// Returns a SephoraSpain struct.
        /// # Arguments
        /// config - The global configuration.
        /// # Returns
        /// Self
        pub fn new(config: &'a Configuration) -> Self {
            Self { config }
        }

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
        ) -> Result<Vec<String>, SearchError> {
            let mut urls: Vec<String> = Vec::new();
            let mut any_results = false;

            // Select the div that wraps the information for every result found.
            let selector = scraper::Selector::parse(
                "#search-result-items>li>div>.product-info-wrapper>.product-info",
            )
            .unwrap();

            let results = document.select(&selector);
            let mut num_results = 0;
            for result in results {
                any_results = true;
                // Check if we reach the maximum number of results
                if num_results != self.config.max_results() {
                    num_results += 1
                } else {
                    break;
                }

                let brand = helper::inner_html_value(&result, "span.product-brand").unwrap();
                let title = helper::attribute_html_value(&result, "h3", "title").unwrap();
                let url = helper::attribute_html_value(&result, "a", "href").unwrap();

                // full_name format = {Brand} {Title} = {Rare Beauty} {Kind Words - Barra de labios mate}
                let full_name = brand + " " + title.as_str();

                let similarity = helper::compare_similarity(name, &full_name);
                if similarity >= self.config.min_similarity() {
                    urls.push(url.to_string());
                } else {
                    eprintln!(
                        "Discarding: {} with {:.2}% of similarity because it is below the minimum",
                        full_name,
                        similarity * 100.0
                    );
                }
            }

            if any_results && !urls.is_empty() {
                Ok(urls)
            } else if any_results && urls.is_empty() {
                Err(SearchError::NotEnoughSimilarity)
            } else {
                Err(SearchError::NotFound)
            }
        }

        /// Creates and initialize the product object.
        /// # Arguments
        /// document - The HTML document for the product to create.
        /// # Returns
        /// Product - The product created based on this HTML webpage.
        fn create_product(document: &Html) -> Product {
            let mut product = Product::default();
            let html = document
                .select(&scraper::Selector::parse("html").unwrap())
                .next()
                .unwrap();

            let name = helper::attribute_html_value(&html, "h1>meta", "content").unwrap();
            println!("{name}");
            product.set_name(name);

            let brand = helper::inner_html_value(&html, "span.brand-name>a").unwrap();
            product.set_brand(brand);

            let mut tones: Vec<Tone> = vec![];
            match html
                .select(
                    &scraper::Selector::parse(
                        r#"div#colorguide-colors>div.colorguide-variations-list"#,
                    )
                    .unwrap(),
                )
                .next()
            {
                // If Some, we have a tones list of elements.
                Some(variations_list) => {
                    // Collect the list of divs for all the tones containing its own data each.
                    let tones_list: Vec<ElementRef> = variations_list
                        .select(&scraper::Selector::parse("div.variation-button-line").unwrap())
                        .collect();

                    // Iterate over all the available tones.
                    // TODO: Check if the tone is sold out and don't add it or add it with a boolean indicating it.
                    for tone_element in tones_list.iter() {
                        let tone_name =
                            helper::inner_html_value(tone_element, "span.variation-title")
                                .unwrap()
                                .trim()
                                .to_string();

                        // Tone price standard and price sale
                        // NOTE: It has different layout if the product its on sale or not
                        let (price_standard, price_sale) = match tone_element
                            .select(
                                &scraper::Selector::parse(".price-sales-standard>span").unwrap(),
                            )
                            .next()
                        {
                            // If Some, it is not on sale.
                            Some(price_standard) => (
                                helper::parse_price_string(price_standard.inner_html()),
                                None,
                            ),
                            // If None, it is on sale.
                            None => {
                                let price_standard =
                                    helper::inner_html_value(tone_element, "span.price-standard")
                                        .unwrap();
                                let price_sale =
                                    helper::inner_html_value(tone_element, "span.price-sales>span")
                                        .unwrap();
                                (
                                    helper::parse_price_string(price_standard),
                                    Some(helper::parse_price_string(price_sale)),
                                )
                            }
                        };

                        // TODO: Find if the product is available. Right know we basically don't add it to the list.
                        tones.push(Tone::new(tone_name, price_standard, price_sale, true));
                    }
                }
                // If None, this product does not have any tones.
                None => (),
            }
            product.set_tones(if tones.is_empty() { None } else { Some(tones) });

            // TODO: Rating of the product.

            // product.set_rating(Some(helper::normalized_rating(rating.parse::<f32>().unwrap(), MAX_RATING)));
            product
        }
    }

    /// Scrappable trait implementation for SephoraSpain
    impl<'a> Scrappable for SephoraSpain<'a> {
        fn look_for_products(&self, name: String) -> Result<Vec<Product>, SearchError> {
            // We receive a word like "This word" and we should search in format of "This+word".
            let formatted_name = name.replace(' ', "+");
            let query = format!("{URL}{SEARCH_SUFFIX}{formatted_name}");
            println!("GET: {query}");

            // If the name match exactly, SephoraSpain redirects you to the product page.
            let response = reqwest::blocking::get(&query).unwrap();
            let response_url = response.url().to_owned();
            let document = scraper::Html::parse_document(&response.text().unwrap());
            let mut products = Vec::<Product>::new();

            // If it only find 1 result it redirects to a product page directly with /p/product_link.html
            if response_url.as_str().contains("/p/") {
                println!("GET: {}", response_url);
                let mut product = SephoraSpain::create_product(&document);
                product.set_link(response_url.to_string());
                let full_name = format!("{} {}", product.brand(), product.name());
                product.set_similarity(helper::compare_similarity(
                    full_name.as_str(),
                    name.as_str(),
                ));
                products.push(product);
            } else {
                // Get the urls for all the coincidence we found in the search with the given `name`
                let products_urls = self.search_results_urls(&document, name.as_str())?;
                println!("Found {} results", products_urls.len());

                // Use threads to perform concurrency when sending petitions.
                let mut handles = Vec::<JoinHandle<Product>>::new();
                for url in products_urls {
                    // Make a copy to be able to send via threads.
                    let name_copy = name.clone();
                    handles.push(thread::spawn(move || {
                        println!("GET: {url}");
                        let response = reqwest::blocking::get(&url).unwrap().text().unwrap();
                        let document = scraper::Html::parse_document(&response);
                        let mut product: Product = SephoraSpain::create_product(&document);
                        product.set_link(url);
                        let full_name = format!("{} {}", product.brand(), product.name());
                        product.set_similarity(helper::compare_similarity(
                            full_name.as_str(),
                            name_copy.as_str(),
                        ));
                        product
                    }));
                }
                for handle in handles {
                    products.push(handle.join().unwrap());
                }
            }

            Ok(products)
        }
    }
}
