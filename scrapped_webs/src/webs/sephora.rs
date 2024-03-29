//! This file encapsulate the different sephoras in the world

use std::thread;
use std::thread::JoinHandle;

use crate::configuration::Configuration;
use crate::helper::{scrapping, utilities};
use crate::{
    product::Product,
    product::Tone,
    scrappable::{Scrappable, SearchError},
};
use scraper::ElementRef;
use scraper::Html;

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
    use anyhow;

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
        fn look_for_products(&self, name: String) -> Result<Vec<Product>, anyhow::Error> {
            // We receive a word like "This word" and we should search in format of "This+word".
            let formatted_name = name.replace(' ', "+");
            let query = format!("{URL}{SEARCH_SUFFIX}{formatted_name}");

            // If the name match exactly, SephoraSpain redirects you to the product page.
            let response = reqwest::blocking::get(query)?;
            let response_url = response.url().to_owned();
            let document = scraper::Html::parse_document(&response.text()?);
            let mut products = Vec::<Product>::new();

            // If it only find 1 result it redirects to a product page directly with /p/product_link.html
            if response_url.as_str().contains("/p/") {
                let mut product = SephoraSpain::create_product(&document);
                product.link = response_url.to_string();
                let full_name = format!("{} {}", product.brand.as_ref().unwrap(), product.name);
                product.similarity =
                    utilities::compare_similarity(full_name.as_str(), name.as_str());
                products.push(product);
            } else {
                // Get the urls for all the coincidence we found in the search with the given `name`
                let products_urls = self.search_results_urls(&document, name.as_str())?;

                // Use threads to perform concurrency when sending petitions.
                let mut handles = Vec::<JoinHandle<Option<Product>>>::new();
                for url in products_urls {
                    let name_copy = name.clone();
                    handles.push(
                        thread::Builder::new()
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
                                let mut product: Product = SephoraSpain::create_product(&document);
                                product.link = url;
                                let full_name =
                                    format!("{} {}", product.brand.as_ref().unwrap(), product.name);
                                product.similarity = utilities::compare_similarity(
                                    full_name.as_str(),
                                    name_copy.as_str(),
                                );
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
            }

            Ok(products)
        }

        fn search_results_urls(
            &self,
            document: &Html,
            name: &str,
        ) -> Result<Vec<String>, anyhow::Error> {
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
                any_results = true;
                let brand = scrapping::inner_html_value(&item, "span.product-brand")
                    .unwrap_or_else(|err| {
                        eprintln!("Brand not found, assigning String::new(): {:?}", err);
                        String::new()
                    });

                let title =
                    scrapping::attribute_html_value(&item, "h3", "title").unwrap_or_else(|err| {
                        eprintln!("Title not found, assigning String::new(): {:?}", err);
                        String::new()
                    });

                let url =
                    scrapping::attribute_html_value(&item, "a", "href").unwrap_or_else(|err| {
                        eprintln!("URL not found, assigning String::new(): {:?}", err);
                        String::new()
                    });

                // full_name format = {Brand} {Title} = {Rare Beauty} {Kind Words - Barra de labios mate}
                let full_name = brand + " " + title.as_str();
                let similarity = utilities::compare_similarity(name, &full_name);

                if similarity >= self.config.min_similarity() && !url.is_empty() {
                    urls.push(url.to_string());
                    num_results += 1;
                    if num_results == self.config.max_results() {
                        break;
                    }
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

        fn create_product(document: &Html) -> Product {
            let mut product = Product::default();
            let html = document.root_element();

            product.name = scrapping::attribute_html_value(&html, "h1>meta", "content")
                .unwrap_or_else(|err| {
                    eprintln!("Product.name not found, assigning String::new(): {:?}", err);
                    String::new()
                });

            product.brand = scrapping::inner_html_value(&html, "span.brand-name")
                .map(|brand| brand.trim().to_string())
                .ok(); // unwrap_or_else is not needed because the None case is already handled by ok() method

            let mut tones: Vec<Tone> = vec![];
            if let Some(variations_list) = html
                .select(
                    &scraper::Selector::parse(
                        r#"div#colorguide-colors>div.colorguide-variations-list"#,
                    )
                    .unwrap(),
                )
                .next()
            {
                // If Some, we have a list of tones.
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
            product.tones = if tones.is_empty() { None } else { Some(tones) };

            // FIXME: It is getting the number of reviews instead of the rating.
            product.rating =
                scrapping::inner_html_value(&html, "span.bv-secondary-rating-summary-rating")
                    .map_or_else(
                        |err| {
                            eprintln!("Product.rating not found, assigning None: {:?}", err);
                            None
                        },
                        |rating| {
                            Some(utilities::normalized_rating(
                                rating.parse::<f32>().unwrap(),
                                MAX_RATING,
                            ))
                        },
                    );

            product
        }

        fn create_tone(element: &ElementRef) -> Tone {
            let tone_name = scrapping::inner_html_value(element, "div.variation-title")
                .map_or_else(
                    |err| {
                        eprintln!("Tone.name not found, assigning None: {:?}", err);
                        None
                    },
                    |tone_name| Some(tone_name.trim().to_string()),
                );
            let available = scrapping::has_html_selector(element, "span.dot-green");

            let price_standard = scrapping::inner_html_value(element, "span.price-sales")
                .map_or_else(
                    |err| {
                        eprintln!("Tone.price_standard not found, assigning None: {:?}", err);
                        None
                    },
                    |text| {
                        // At this moment when we retrieve this element value we have 4 \n and the value is second.
                        let mut price = text.split('\n').nth(1).unwrap().to_string();
                        if price == "\n" || price == "N/A" || price.is_empty() {
                            price = String::from("0 €");
                        }
                        Some(utilities::parse_price_string(price))
                    },
                );

            // price_standard could also be inside span.price-sales this is why later we check if it is greater than price_sale
            //TODO: Not on sales promotions on sephora right now to test this.
            let price_sale = None;
            // if let Some(price_standard) = price_standard {
            //     price_sale = match scrapping::inner_html_value(element, "TODO-get-price_sale") {
            //         Ok(price_sale) => {
            //             let price_sale_number = utilities::parse_price_string(price_sale);
            //             if price_standard > price_sale_number {
            //                 Some(price_sale_number)
            //             } else {
            //                 None
            //             }
            //         }
            //         Err(err) => {
            //             eprintln!("Tone.price_sale not found, assigning None: {:?}", err);
            //             None
            //         }
            //     }
            // };

            Tone::new(tone_name, price_standard, price_sale, available, None, None)
        }
    }
}
