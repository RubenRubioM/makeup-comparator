//! Module for maquillalia.com

/* maquillalia.com logic explanation
1 - We search throw the standard search endpoint "search.php?buscar=" to retrieve the results.
2 - We retrieve the products in a grid of 20 products per page.
    2.1 - To retrieve more products we have to make more petitions with "&page=n" until we reach the last product.
3 - We can retrieve the grid with the product items with the selector "div.ListProds>div".
    NOTE: Maquillalia shows also the tones for a product because they are individual URLS, we will filter those and treat them at product page level.
    3.1 - To filter them we get the title in format {Brand} - {Name} - {Tone} and format it to remove the tone part and only store the first of them.
            This way if we find 4 items that are 1 but with four different tones, we will only store the first one and redirect to it.
    3.2 - We get the full name with "h3.Title>a" and the URL with the "href" attribute.
*/

use std::thread::{self, JoinHandle};

use crate::configuration::Configuration;
use crate::helper;
use crate::product::{Product, Tone};
use crate::scrappable::{Scrappable, SearchError};
use ansi_term::Colour::RGB;

// Webpage url.
const URL: &str = "https://www.maquillalia.com/";
// Suffix for searching in the website.
const SEARCH_SUFFIX: &str = "search.php?buscar=";
// Maximum rating for SephoraSpain.
const _MAX_RATING: f32 = 5.0;

/// Structure that define functionality for SephoraSpain.
pub struct Maquillalia<'a> {
    pub config: &'a Configuration,
}

/// Implementation for Maquillalia structure.
impl<'a> Maquillalia<'a> {
    pub fn new(config: &'a Configuration) -> Self {
        Self { config }
    }
}

/// Scrappable trait implementation for Maquillalia.
impl<'a> Scrappable for Maquillalia<'a> {
    fn look_for_products(&self, name: String) -> Result<Vec<Product>, SearchError> {
        // We receive a word like "This word" and we should search in format of "This+word".
        let formatted_name = name.replace(' ', "+");
        let query = format!("{URL}{SEARCH_SUFFIX}{formatted_name}");
        println!("GET: {query}");

        // If the name match exactly, SephoraSpain redirects you to the product page.
        let response = reqwest::blocking::get(&query).unwrap();
        let document = scraper::Html::parse_document(&response.text().unwrap());
        let mut products = Vec::<Product>::new();

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
                let mut product: Product = Self::create_product(&document);
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
        Ok(products)
    }

    // TODO: Support for products in more pages. You can find more results by concat &page=n. It shows 20 products per page.
    // Note: If we try to request a non existing number of page, e.g: &page=10000, it gets redirected to the last page aswell. We will have to check when we are at the last page.
    fn search_results_urls(
        &self,
        document: &scraper::Html,
        name: &str,
    ) -> Result<Vec<String>, SearchError> {
        let mut urls: Vec<String> = Vec::new();
        let mut any_results = false;

        let products_grid_selector = scraper::Selector::parse("div.ListProds>div").unwrap();
        // Select the div that wraps the information for every result found.
        let items = document.select(&products_grid_selector);
        let mut num_results = 0;
        // The name of products to store only one and skip the next's.
        let mut individual_products: Vec<String> = Vec::new();

        for item in items {
            // In the search page we have all the tones for a product so we will only store one of them and skip the rest because they are separated in the las dash({Brand} - {Name} - {Tone}).
            // Name format is {Brand} - {Name} - {Tone}
            let element_name = helper::inner_html_value(&item, "h3.Title>a").unwrap();
            let mut splitted_name = element_name.split('-');
            let full_name =
                splitted_name.next().unwrap().to_string() + splitted_name.next().unwrap();
            let url = helper::attribute_html_value(&item, "h3.Title>a", "href").unwrap();

            let similarity = helper::compare_similarity(name, &full_name);
            if similarity >= self.config.min_similarity() {
                // If we already have the product name, we skip the product because must be a tone of that product.
                if individual_products.contains(&full_name) {
                    continue;
                }

                individual_products.push(full_name);
                urls.push(url.to_string());
                any_results = true;
                num_results += 1;
                if num_results == self.config.max_results() {
                    break;
                }
            } else {
                println!(
                    "{}",
                    RGB(255, 121, 0).normal().paint(format!(
                        "Discarding [{:.2}%]: {}",
                        similarity * 100.0,
                        full_name
                    ))
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

    fn create_product(_document: &scraper::Html) -> Product {
        Product::default()
    }

    fn create_tone(_element: &scraper::ElementRef) -> Tone {
        Tone::default()
    }
}
