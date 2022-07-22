pub mod sephora_spain {

    use crate::common;
    use crate::helper;
    use crate::{
        product::Product,
        scrappable::{Scrappable, SearchError},
    };
    use scraper::Html;

    /// Blank struct to define the functionality for SephoraSpain.
    pub struct SephoraSpain;

    const URL: &str = "http://www.sephora.es/";
    const SEARCH_SUFFIX: &str = "buscar?q=";

    /// Returns the url of the products found.
    fn products(document: &Html, name: &str) -> Result<Vec<String>, SearchError> {
        let mut urls: Vec<String> = Vec::new();
        let mut any_results = false;
        // Select the div that wraps the information for every result found.
        let selector = scraper::Selector::parse(
            "#search-result-items>li>div>.product-info-wrapper>.product-info",
        )
        .unwrap();

        let results = document.select(&selector);

        for result in results {
            any_results = true;
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
            let full_name = brand + title;

            // Unsafe: using a mutable static variable (common::MIN_SIMILARITY).
            unsafe {
                if helper::compare_similarity(name, &full_name) > common::MIN_SIMILARITY {
                    urls.push(url.to_string());
                }
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

    /// Scrappable trait implementation for SephoraSpain
    impl Scrappable for SephoraSpain {
        fn look_for_products(name: &str) -> Result<Vec<Product>, SearchError> {
            // We recieve a word like "This word" and we should search in format of "This+word".
            let formatted_name = name.replace(' ', "+");
            let query = format!("{URL}{SEARCH_SUFFIX}{formatted_name}");
            println!("GET: {}", query);

            let response = reqwest::blocking::get(query).unwrap().text().unwrap();
            let document = scraper::Html::parse_document(&response);

            let _products_urls = products(&document, name).unwrap();
            Ok(Vec::new())
        }
    }
}
