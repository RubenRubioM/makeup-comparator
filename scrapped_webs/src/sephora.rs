pub mod sephora_spain {
    use std::thread;
    use std::thread::JoinHandle;

    use crate::configuration;
    use crate::helper;
    use crate::{
        product::Product,
        product::Tone,
        scrappable::{Scrappable, SearchError},
    };
    use scraper::ElementRef;
    use scraper::Html;

    // Alias for Url variables.
    type Url = String;
    // Webpage url.
    const URL: &str = "https://www.sephora.es/";
    // Suffix for searching in the website.
    const SEARCH_SUFFIX: &str = "buscar?q=";
    // Maximum rating for SephoraSpain.
    // const MAX_RATING: f32 = 5.0;

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
                .expect("not found any ElementRef with .product-info>span.product-brand")
                .inner_html();
            // Find the product title.
            let title = result
                .select(&scraper::Selector::parse("h3").unwrap())
                .next()
                .expect("not found any ElementRef with .product-info>h3")
                .value()
                .attr("title")
                .expect("attribute (title) inside .product-info>h3 not found");
            // Find the url.
            let url = result
                .select(&scraper::Selector::parse("a").unwrap())
                .next()
                .expect("not found any ElementRef with .product-info>a")
                .value()
                .attr("href")
                .expect("attribute (href) inside .product-info>a not found");

            // full_name format = {Brand} {Title} = {Rare Beauty} {Kind Words - Barra de labios mate}
            let full_name = brand + " " + title;

            // Unsafe: using a mutable static variable (common::MIN_SIMILARITY).
            let similarity = helper::compare_similarity(name, &full_name);
            if similarity >= unsafe { configuration::MIN_SIMILARITY } {
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
            Err(SearchError::NotEnoughtSimilarity)
        } else {
            Err(SearchError::NotFound)
        }
    }

    /// Creates and initialize the product objetct.
    // TODO: Error handling for all posibilities of layouts in sephora.
    fn create_product(document: &Html) -> Product {
        let mut product = Product::default();

        // Name of the product
        // TODO: It is not reporting properly the name of the product in h1>meta
        let name = document
            .select(&scraper::Selector::parse("h1>meta").unwrap())
            .next()
            .expect("not found any ElementRef with h1>meta")
            .value()
            .attr("content")
            .unwrap()
            .to_string();
        product.set_name(name);

        // Brand name
        let brand = document
            .select(&scraper::Selector::parse("h1>div>span.brand-name>a").unwrap())
            .next()
            .expect("not found any ElementRef with h1>div>span.brand-name>a")
            .inner_html();
        product.set_brand(brand);

        // Tones
        let mut tones: Vec<Tone> = vec![];
        match document
            .select(
                &scraper::Selector::parse("div#colorguide-colors>div.colorguide-variations-list")
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

                // Iterate over all the avaliable tones.
                for tone in tones_list.iter() {
                    // Tone name
                    let tone_name = tone
                        .select(&scraper::Selector::parse("span.variation-title").unwrap())
                        .next()
                        .expect("not found any ElementRef with span.variation-title")
                        .inner_html();

                    // Tone price standard and price sale
                    // NOTE: It has different layout if the product its on sale or not
                    let (price_standard, price_sale) = match tone
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
                            let price_standard = tone
                                .select(&scraper::Selector::parse("span.price-standard").unwrap())
                                .next()
                                .unwrap()
                                .inner_html();
                            let price_sale = tone
                                .select(&scraper::Selector::parse("span.price-sales>span").unwrap())
                                .next()
                                .unwrap()
                                .inner_html();
                            (
                                helper::parse_price_string(price_standard),
                                Some(helper::parse_price_string(price_sale)),
                            )
                        }
                    };
                    tones.push(Tone::new(tone_name, price_standard, price_sale));
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

    /// Scrappable trait implementation for SephoraSpain
    impl Scrappable for SephoraSpain {
        fn look_for_products(name: &'static str) -> Result<Vec<Product>, SearchError> {
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
                println!("Found {} results", products_urls.len());

                // Use threads to perform concurrency when sending petitions.
                let mut handles = Vec::<JoinHandle<Product>>::new();
                for url in products_urls {
                    handles.push(thread::spawn(move || {
                        println!("GET: {url}");
                        let response = reqwest::blocking::get(&url).unwrap().text().unwrap();
                        let document = scraper::Html::parse_document(&response);
                        let mut product: Product = create_product(&document);
                        product.set_link(url);
                        let full_name = format!("{} {}", product.brand(), product.name());
                        product
                            .set_similarity(helper::compare_similarity(full_name.as_str(), name));
                        product
                    }));
                }
                for handle in handles {
                    products.push(handle.join().unwrap());
                }
            } else {
                let mut product = create_product(&document);
                product.set_link(response_url.to_string());
                let full_name = format!("{} {}", product.brand(), product.name());
                product.set_similarity(helper::compare_similarity(full_name.as_str(), name));
                products.push(product);
            }

            Ok(products)
        }
    }
}
