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

use ansi_term::Colour::RGB;

/* sephora.es logic explanation
1 - We search throw the standard search endpoint "buscar?q=" to retrieve the results.
2 - When we get redirected to the searched products we have a grid with all the products found.
    2.1 - If we found only 1 result we got redirected to the product page.
    2.2 - If we found for specific category such as "barra de labios" we got redirected to an specific url too but with the same logic that when we find results.
3 - We collect the urls for specifics products throw de <href> in the grid.
4 - We then make a petition to go to the specific url for every product found.
5 - Once we are in the specific product page "https://www.sephora.es/p/NAME.html"
    5.1 - Retrieve the "name" of the product from the attribute "content" inside the "h1>meta".
    5.2 - Retrieve the "brand" of the product from the inner html in "span.brand-name>a".
    5.2 - Retrieve the tones from a submenu in "div#colorguide-colors>div.colorguide-variations-list".
        5.2.1 - If this returns None, the product do not have any tones available.
        5.2.2 - If this returns Some, we iterate for every tone.
            5.2.2.1 - Retrieve the "name" of the product from the inner html in "span.variation-title".
            5.2.2.2 - If we find the element ".price-sales-standard>span" then the element is not on sale and we add this to the "price_standard".
            5.2.2.3 - If we do not find the element ".price-sales-standard>span" then the element is on sale and we find the "price_standard" in "span.price-standard" and the "price_sales" in "span.price-sales>span".
    5.3 - Finally, we retrieve the "rating" from the element "div.bv_numReviews_text>span". If it returns an empty string is because there is no reviews yet.
*/
/// Module for sephora.es
pub mod spain {
    use super::*;
    // Webpage url.
    const URL: &str = "https://www.sephora.es/";
    // Suffix for searching in the website.
    const SEARCH_SUFFIX: &str = "buscar?q=";
    // Maximum rating for SephoraSpain.
    const MAX_RATING: f32 = 5.0;

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
    }

    /// Scrappable trait implementation for SephoraSpain.
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

            let items = document.select(&selector);
            let mut num_results = 0;

            for item in items {
                let brand = helper::inner_html_value(&item, "span.product-brand").unwrap();
                let title = helper::attribute_html_value(&item, "h3", "title").unwrap();
                let url = helper::attribute_html_value(&item, "a", "href").unwrap();

                // full_name format = {Brand} {Title} = {Rare Beauty} {Kind Words - Barra de labios mate}
                let full_name = brand + " " + title.as_str();
                let similarity = helper::compare_similarity(name, &full_name);

                if similarity >= self.config.min_similarity() {
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

        fn create_product(document: &Html) -> Product {
            let mut product = Product::default();
            let html = document.root_element();

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
                        tones.push(Self::create_tone(tone_element));
                    }
                }
                // If None, this product does not have any tones.
                None => (),
            }
            product.set_tones(if tones.is_empty() { None } else { Some(tones) });

            let mut rating =
                helper::attribute_html_value(&html, "div.bv_numReviews_text>span>meta", "content")
                    .unwrap();
            rating = if rating.is_empty() {
                "0.0".to_string()
            } else {
                rating
            };
            product.set_rating(Some(helper::normalized_rating(
                rating.parse::<f32>().unwrap(),
                MAX_RATING,
            )));

            product
        }

        fn create_tone(element: &ElementRef) -> Tone {
            // TODO: Find if the product is available. Right know we basically don't add it to the list.
            let tone_name = helper::inner_html_value(element, "span.variation-title")
                .unwrap()
                .trim()
                .to_string();

            // Tone price standard and price sale
            // NOTE: It has different layout if the product its on sale or not
            let (price_standard, price_sale) = match element
                .select(&scraper::Selector::parse(".price-sales-standard>span").unwrap())
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
                        helper::inner_html_value(element, "span.price-standard").unwrap();
                    let price_sale =
                        helper::inner_html_value(element, "span.price-sales>span").unwrap();
                    (
                        helper::parse_price_string(price_standard),
                        Some(helper::parse_price_string(price_sale)),
                    )
                }
            };
            Tone::new(tone_name, price_standard, price_sale, true, None, None)
        }
    }
}
