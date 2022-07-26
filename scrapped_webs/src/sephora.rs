pub mod sephora_spain {
    use std::thread;
    use std::thread::JoinHandle;

    use crate::configuration;
    use crate::helper;
    use crate::{
        product::Product,
        scrappable::{Scrappable, SearchError},
    };
    use scraper::Html;

    // Alias for Url variables.
    type Url = String;
    // Webpage url.
    const URL: &str = "https://www.sephora.es/";
    // Suffix for searching in the website.
    const SEARCH_SUFFIX: &str = "buscar?q=";

    /// Blank struct to define the functionality for SephoraSpain.
    pub struct SephoraSpain;

    /// Returns the url of the products found.
    fn search_results_urls(document: &Html, name: &str) -> Result<Vec<Url>, SearchError> {
        let mut urls: Vec<Url> = Vec::new();
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
            if num_results != unsafe { configuration::MAX_RESULTS } {
                num_results += 1
            } else {
                break;
            }

            // Find the brand inside the HTML.
            let brand = result
                .select(&scraper::Selector::parse("span.product-brand").unwrap())
                .next()
                .unwrap()
                .inner_html();
            // Find the product title.
            let title = result
                .select(&scraper::Selector::parse("h3").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("title")
                .unwrap();
            // Find the url.
            let url = result
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .unwrap()
                .value()
                .attr("href")
                .unwrap();

            // full_name format = {Brand} {Title} = {Rare Beauty} {Kind Words - Barra de labios mate}
            let full_name = brand + " " + title;

            // Unsafe: using a mutable static variable (common::MIN_SIMILARITY).
            let similarity = helper::compare_similarity(name, &full_name);
            if similarity >= unsafe { configuration::MIN_SIMILARITY } {
                urls.push(url.to_string());
            } else {
                println!(
                    "Discarding: {} with {:.2}% of similarity because it is below the minimum",
                    full_name,
                    similarity * 100.0
                );
            }
        }

        if any_results && !urls.is_empty() {
            Ok(urls)
        } else if any_results && urls.is_empty() {
            Err(SearchError::NotEnoughtSimilarity)
        } else {
            Err(SearchError::NotFound)
        }
    }

    fn create_product(_document: &Html) -> Product {

        Product::default()
    }

    /// Scrappable trait implementation for SephoraSpain
    impl Scrappable for SephoraSpain {
        fn look_for_products(name: &str) -> Result<Vec<Product>, SearchError> {
            // We recieve a word like "This word" and we should search in format of "This+word".
            let formatted_name = name.replace(' ', "+");
            let query: Url = format!("{URL}{SEARCH_SUFFIX}{formatted_name}");
            println!("GET: {query}");

            // If the name match exactly, SephoraSpain redirects you to the product page.
            let response = reqwest::blocking::get(&query).unwrap();
            let response_url = response.url().to_owned();
            let document = scraper::Html::parse_document(&response.text().unwrap());
            let mut products = Vec::<Product>::new();
            if response_url.as_str() == query {
                // Get the urls for all the coincidence we found in the search with the given `name`
                let products_urls: Vec<Url> = search_results_urls(&document, name)?;
    
                // Use threads to perform concurrency when sending petitions.
                let mut handles = Vec::<JoinHandle<Product>>::new();
                for url in products_urls {
                    handles.push(thread::spawn(move || {
                        let response = reqwest::blocking::get(&url).unwrap().text().unwrap();
                        let document = scraper::Html::parse_document(&response);
                        let mut product: Product = create_product(&document);
                        product.set_link(url);
                        product
                    }));
                }
                for handle in handles {
                    products.push(handle.join().unwrap());
                }
                
            } else {
                let mut product = create_product(&document);
                product.set_link(response_url.to_string());
                products.push(product);
            }

            Ok(products)
        }
    }
}
