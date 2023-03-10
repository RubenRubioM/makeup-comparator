//! Module for maquillalia.com

/* maquillalia.com logic explanation
1 - We search throw the standard search endpoint "search.php?buscar=" to retrieve the results.
2 - We retrieve the products in a grid of 20 products per page.
    2.1 - To retrieve more products we have to make more petitions with "&page=n" until we reach the last product.
    2.2 - We check if we reached the final page checking the number of products in "div.NumPro>strong". If we are at this page we break the loop.
3 - We can retrieve the grid with the product items with the selector "div.ListProds>div".
    NOTE: Maquillalia shows also the tones for a product because they are individual URLS, we will filter those and treat them at product page level.
    3.1 - We detect if we do not have any results if we have the element "div.msje-wrng>div.msje-icon".
    3.2 - To filter them we get the title in format {Brand} - {Name} - {Tone} and format it to remove the tone part and only store the first of them.
            This way if we find 4 items that are 1 but with four different tones, we will only store the first one and redirect to it.
    3.3 - We get the full name with "h3.Title>a" and the URL with the "href" attribute.
*/

use std::thread::JoinHandle;

use crate::configuration::Configuration;
use crate::helper::{scrapping, utilities};
use crate::product::{Product, Tone};
use crate::scrappable::{Scrappable, SearchError};

// Webpage url.
const URL: &str = "https://www.maquillalia.com/";
// Suffix for searching in the website.
const SEARCH_SUFFIX: &str = "search.php?buscar=";
// Suffix for pagination.
const PAGINATION_SUFFIX: &str = "page=";
// Items showing per page used to determine if we reach the last page of products.
const ITEMS_PER_PAGE: usize = 20;
// Maximum rating for SephoraSpain.
const MAX_RATING: f32 = 5.0;

/// Structure that define functionality for SephoraSpain.
pub struct Maquillalia<'a> {
    pub config: &'a Configuration,
}

/// Implementation for Maquillalia structure.
impl<'a> Maquillalia<'a> {
    /// Creates a new Maquillalia instance.
    pub fn new(config: &'a Configuration) -> Self {
        Self { config }
    }
    /// Returns the name of a product without the tone name on it.
    /// # Example
    /// let v: String = get_name_without_tone(String::from("Maybelline - Labial líquido SuperStay Vinyl Ink - 35: Pink"));
    /// assert_eq!(v, "Maybelline - Labial líquido SuperStay Vinyl Ink");
    pub fn get_name_without_tone(full_name: &str) -> String {
        let mut splitted_name = full_name.split('-');
        let full_name =
            splitted_name.next().unwrap().to_string() + splitted_name.next().unwrap().trim_end();
        full_name.replace("  ", " - ")
    }
    /// Returns the name of the tone.
    pub fn get_tone_name(full_name: &str) -> String {
        let mut splitted_name = full_name.split('-');
        splitted_name.next().unwrap();
        splitted_name.next().unwrap();
        splitted_name.collect()
    }
}

/// Scrappable trait implementation for Maquillalia.
impl<'a> Scrappable for Maquillalia<'a> {
    fn look_for_products(&self, name: String) -> Result<Vec<Product>, anyhow::Error> {
        // We receive a word like "This word" and we should search in format of "This+word".
        let formatted_name = name.replace(' ', "+");
        let mut page: usize = 1;
        let mut is_last_page: bool = false;
        let mut products = Vec::<Product>::new();
        let mut products_urls: Vec<String> = vec![];

        // We have to search for all the pages to retrieve the products.
        // TODO: Im not sure if i can parallelize and preserve the order of insertion in the products_urls vector in order by page.
        while !is_last_page {
            let query = format!("{URL}{SEARCH_SUFFIX}{formatted_name}&{PAGINATION_SUFFIX}{page}");

            let response = reqwest::blocking::get(query).unwrap();
            let document = scraper::Html::parse_document(&response.text().unwrap());
            let total_results =
                scrapping::inner_html_value(&document.root_element(), "div.NumPro>strong").unwrap();
            // Get the urls for all the coincidence we found in the search with the given `name`
            let page_products_urls = self.search_results_urls(&document, name.as_str())?;
            for product_url in page_products_urls {
                products_urls.push(product_url);
            }

            page += 1;
            let actual_items = page * ITEMS_PER_PAGE;
            if self.config.max_results() <= products_urls.len()
                || actual_items >= total_results.parse().unwrap()
            {
                is_last_page = true;
            }

            while products_urls.len() > self.config.max_results() {
                products_urls.pop();
            }
        }

        // Use threads to perform concurrency when sending petitions.
        let mut handles = Vec::<JoinHandle<Option<Product>>>::new();
        for url in products_urls {
            // Make a copy to be able to send via threads.
            let name_copy = name.clone();
            handles.push(
                std::thread::Builder::new()
                    .name(url.clone())
                    .spawn(move || -> Option<Product> {
                        let response: String;
                        // TODO: Maybe add some logging in case of returning None
                        if let Ok(http_response) = reqwest::blocking::get(&url) {
                            if let Ok(text) = http_response.text() {
                                response = text;
                            } else {
                                return None;
                            }
                        } else {
                            return None;
                        }
                        let document = scraper::Html::parse_document(&response);
                        let mut product: Product = Self::create_product(&document);
                        product.link = url;
                        let full_name =
                            format!("{} {}", product.brand.as_ref().unwrap(), product.name);
                        product.similarity =
                            utilities::compare_similarity(full_name.as_str(), name_copy.as_str());
                        Some(product)
                    })
                    .unwrap(),
            );
        }
        for handle in handles {
            if let Some(product) = handle.join().unwrap() {
                products.push(product);
            }
        }
        Ok(products)
    }

    fn search_results_urls(
        &self,
        document: &scraper::Html,
        name: &str,
    ) -> Result<Vec<String>, anyhow::Error> {
        let mut urls: Vec<String> = Vec::new();
        let mut any_results = false;

        // Check if we find the flag that indicates that we did not find any results.
        if scrapping::inner_html_value(&document.root_element(), "div.msje-wrng>div.msje-icon")
            .is_ok()
        {
            return Err(anyhow::anyhow!(SearchError::NotFound));
        }

        let products_grid_selector = scraper::Selector::parse("div.ListProds>div").unwrap();
        // Select the div that wraps the information for every result found.
        let items = document.select(&products_grid_selector);
        // The name of products to store only one and skip the next's.
        let mut individual_products: Vec<String> = Vec::new();

        for item in items {
            // In the search page we have all the tones for a product so we will only store one of them and skip the rest because they are separated in the las dash({Brand} - {Name} - {Tone}).
            // Name format is {Brand} - {Name} - {Tone}
            let element_name = scrapping::inner_html_value(&item, "h3.Title>a").unwrap();
            let full_name = Maquillalia::get_name_without_tone(&element_name);
            let url = scrapping::attribute_html_value(&item, "h3.Title>a", "href").unwrap();
            any_results = true;

            let similarity = utilities::compare_similarity(name, &full_name);
            if similarity >= self.config.min_similarity() {
                // If we already have the product name, we skip the product because must be a tone of that product.
                if individual_products.contains(&full_name) {
                    continue;
                }

                individual_products.push(full_name);
                urls.push(url.to_string());
            }
        }

        if any_results && !urls.is_empty() {
            Ok(urls)
        } else if any_results && urls.is_empty() {
            Err(anyhow::anyhow!(SearchError::NotEnoughSimilarity))
        } else {
            Err(anyhow::anyhow!(SearchError::NotFound))
        }
    }

    fn create_product(document: &scraper::Html) -> Product {
        let mut product = Product::default();
        let html = document.root_element();

        let full_name = Maquillalia::get_name_without_tone(
            &scrapping::inner_html_value(&html, "h1.Title").unwrap(),
        );
        // TODO: Remove trailing and beginning white spaces.
        let mut name_and_brand = full_name.trim().split('-');
        // TODO: Update this some with proper error handling
        product.brand = Some(name_and_brand.next().unwrap().to_string());
        product.name = name_and_brand.next().unwrap().to_string();

        // If we find the element for different tones we iterate over all the websites and fill the Tone variable.
        let tones_urls_selector = scraper::Selector::parse("ul.familasColores>li").unwrap();
        // Select the div that wraps the information for every result found.
        let tones_urls = document.select(&tones_urls_selector);
        for url in tones_urls {
            // TODO: Try to parallelize in the future.
            let url_string = scrapping::attribute_html_value(&url, "a", "href").unwrap();
            let response = reqwest::blocking::get(&url_string).unwrap().text().unwrap();
            let document = scraper::Html::parse_document(&response);
            let mut tone = Self::create_tone(&document.root_element());
            tone.url = Some(url_string);
            product.add_tone(tone);
        }

        if product.tones.is_none() {
            if scrapping::has_html_selector(&html, "table>tbody>tr>td>div.Price>del") {
                // It is on sale.
                // TODO: Update this some with proper error handling
                product.price_standard = Some(utilities::parse_price_string(
                    scrapping::inner_html_value(&html, "table>tbody>tr>td>div.Price>del").unwrap(),
                ));
                // TODO: Update this some with proper error handling
                product.price_sales = Some(utilities::parse_price_string(
                    scrapping::inner_html_value(&html, "table>tbody>tr>td>div.Price>strong")
                        .unwrap(),
                ));
            } else {
                // TODO: Update this some with proper error handling
                product.price_standard = Some(utilities::parse_price_string(
                    scrapping::inner_html_value(&html, "table>tbody>tr>td>div.Price>strong")
                        .unwrap(),
                ));
            }
            product.rating = Some(utilities::normalized_rating(
                scrapping::attribute_html_value(&html, "div.Rating>span.Stars", "data-rating")
                    .unwrap()
                    .parse()
                    .unwrap(),
                MAX_RATING,
            ));
        }
        product
    }

    fn create_tone(element: &scraper::ElementRef) -> Tone {
        let tone_name =
            Maquillalia::get_tone_name(&scrapping::inner_html_value(element, "h1.Title").unwrap())
                .trim()
                .to_string();
        let price_standard;
        let mut price_sales = Option::<f32>::None;
        if scrapping::has_html_selector(element, "table>tbody>tr>td>div.Price>del") {
            // It is on sale.
            price_standard = Some(utilities::parse_price_string(
                scrapping::inner_html_value(element, "table>tbody>tr>td>div.Price>del").unwrap(),
            ));
            price_sales = Some(utilities::parse_price_string(
                scrapping::inner_html_value(element, "table>tbody>tr>td>div.Price>strong").unwrap(),
            ));
        } else {
            price_standard = Some(utilities::parse_price_string(
                scrapping::inner_html_value(element, "table>tbody>tr>td>div.Price>strong").unwrap(),
            ));
        }
        let rating = utilities::normalized_rating(
            scrapping::attribute_html_value(element, "div.Rating>span.Stars", "data-rating")
                .unwrap()
                .parse()
                .unwrap(),
            MAX_RATING,
        );
        Tone::new(
            Some(tone_name),
            price_standard,
            price_sales,
            true,
            None,
            Some(rating),
        )
    }
}
