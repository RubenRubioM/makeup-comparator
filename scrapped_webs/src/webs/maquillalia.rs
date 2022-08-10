//! Module for maquillalia.com

use crate::configuration::Configuration;
use crate::product::Product;
use crate::scrappable::{Scrappable, SearchError};

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
        let _response_url = response.url().to_owned();
        let _document = scraper::Html::parse_document(&response.text().unwrap());
        let products = Vec::<Product>::new();

        Ok(products)
    }

    fn create_product(_document: &scraper::Html) -> Product {
        todo!()
    }

    fn search_results_urls(
        &self,
        _document: &scraper::Html,
        _name: &str,
    ) -> Result<Vec<String>, SearchError> {
        todo!()
    }
}
