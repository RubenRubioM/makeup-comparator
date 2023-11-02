//! Handle the scraping of the data from the web page.

use std::collections::HashMap;

use crate::{
    parameters::{self, Args},
    parameters_processor::{self, ParametersProcessor},
};
use clap::Parser;
use scrapped_webs::{
    configuration::Configuration,
    product::Product,
    scrappable::Scrappable,
    webs::{maquillalia::Maquillalia, sephora::spain::SephoraSpain},
};

type ResultsByWebsite = HashMap<parameters::Website, Vec<Product>>;

#[derive(Debug)]
pub struct ScraperHandler {
    /// The configuration for the program.
    parameters_processor: ParametersProcessor,
}

impl ScraperHandler {
    /// Creates a new ScraperHandler.
    /// # Arguments
    /// * `parameters_processor` - The parameters processor.
    /// # Returns
    /// A new ScraperHandler.
    pub fn new(parameters_processor: ParametersProcessor) -> Self {
        Self {
            parameters_processor,
        }
    }

    /// Returns the results of the search.
    /// # Returns
    /// A HashMap with the results of the search.
    /// The key is the website and the value is a vector of products.
    pub fn get_results(&self) -> ResultsByWebsite {
        let mut results_by_website = ResultsByWebsite::new();

        for web in self.parameters_processor.websites().iter() {
            match web {
                parameters::Website::SephoraSpain => {
                    results_by_website.insert(
                        parameters::Website::SephoraSpain,
                        self.get_results_sephora_spain(),
                    );
                }
                parameters::Website::Maquillalia => {
                    results_by_website.insert(
                        parameters::Website::Maquillalia,
                        self.get_results_maquillalia(),
                    );
                }
                parameters::Website::All => {
                    results_by_website.insert(
                        parameters::Website::SephoraSpain,
                        self.get_results_sephora_spain(),
                    );
                    results_by_website.insert(
                        parameters::Website::Maquillalia,
                        self.get_results_maquillalia(),
                    );
                }
            }
        }
        self.sort(&mut results_by_website);
        results_by_website
    }

    /// Sorts the products by the args.sort_by parameter
    /// # Arguments
    /// * `results_by_website` - The products for every shop.
    fn sort(&self, results_by_website: &mut ResultsByWebsite) {
        match self.parameters_processor.sorting_type() {
            parameters::SortingType::Name => {
                results_by_website
                    .iter_mut()
                    .for_each(|(_, results)| results.sort_by_key(|product| product.name.clone()));
            }
            parameters::SortingType::Price => {
                //TODO: This is not working, we have to check the tones prices if we have no price in the product
                results_by_website.iter_mut().for_each(|(_, results)| {
                    results.sort_by(|p1, p2| {
                        // If not found neither price_sales or price_standard, assign a f32::MAX to put it at the end of the vector.
                        let price1 = if p1.price_sales.is_some() {
                            p1.price_sales.unwrap()
                        } else {
                            p1.price_standard.unwrap_or(f32::MAX)
                        };
                        let price2 = if p2.price_sales.is_some() {
                            p2.price_sales.unwrap()
                        } else {
                            p1.price_standard.unwrap_or(f32::MAX)
                        };

                        price2.partial_cmp(&price1).unwrap()
                    })
                });
            }
            parameters::SortingType::Similarity => {
                results_by_website.iter_mut().for_each(|(_, results)| {
                    results.sort_by(|p1, p2| p2.similarity.partial_cmp(&p1.similarity).unwrap())
                })
            }
            parameters::SortingType::Brand => {
                results_by_website
                    .iter_mut()
                    .for_each(|(_, results)| results.sort_by_key(|product| product.brand.clone()));
            }
            parameters::SortingType::Rating => {
                results_by_website.iter_mut().for_each(|(_, results)| {
                    results.sort_by(|p1, p2| {
                        let rating1 = p1.rating.unwrap_or(0_f32);
                        let rating2 = p2.rating.unwrap_or(0_f32);
                        rating2.partial_cmp(&rating1).unwrap()
                    })
                });
            }
        }
    }

    fn get_results_sephora_spain(&self) -> Vec<Product> {
        let sephora_spain = SephoraSpain::new(self.parameters_processor.configuration());
        let products =
            match sephora_spain.look_for_products(self.parameters_processor.product().clone()) {
                Ok(products) => products,
                Err(err) => {
                    eprintln!("{:?}", err);
                    Vec::new()
                }
            };
        products
    }

    fn get_results_maquillalia(&self) -> Vec<Product> {
        let maquillalia = Maquillalia::new(self.parameters_processor.configuration());
        let products =
            match maquillalia.look_for_products(self.parameters_processor.product().clone()) {
                Ok(products) => products,
                Err(err) => {
                    eprintln!("{:?}", err);
                    Vec::new()
                }
            };
        products
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests all the possible sorting.
    /// TODO: Improve this test.
    #[test]
    #[ignore]
    fn sort_all_paths() {
        let mut args = Args {
            product: String::from("labial"),
            max_results: 2,
            min_similarity: 0.0,
            websites: vec![
                parameters::Website::SephoraSpain,
                parameters::Website::Maquillalia,
            ],
            sort_by: parameters::SortingType::Similarity,
        };
        let parameters_processor = ParametersProcessor::new(args.clone());
        let scraper_handler = ScraperHandler::new(parameters_processor);
        // Sort by Similarity
        scraper_handler.get_results();
        // Sort by Name
        args.sort_by = parameters::SortingType::Name;
        let parameters_processor = ParametersProcessor::new(args.clone());
        let scraper_handler = ScraperHandler::new(parameters_processor);
        scraper_handler.get_results();
        // Sort by Price
        args.sort_by = parameters::SortingType::Price;
        let parameters_processor = ParametersProcessor::new(args.clone());
        let scraper_handler = ScraperHandler::new(parameters_processor);
        scraper_handler.get_results();
        // Sort by Brand
        args.sort_by = parameters::SortingType::Brand;
        let parameters_processor = ParametersProcessor::new(args.clone());
        let scraper_handler = ScraperHandler::new(parameters_processor);
        scraper_handler.get_results();
        // Sort by Rating
        args.sort_by = parameters::SortingType::Rating;
        let parameters_processor = ParametersProcessor::new(args.clone());
        let scraper_handler = ScraperHandler::new(parameters_processor);
        scraper_handler.get_results();
    }

    /// Tests a search for a product in two websites.
    #[test]
    #[ignore]
    fn get_results() {
        let args = Args {
            product: String::from("labial"),
            max_results: 15,
            min_similarity: 0.0,
            websites: vec![
                parameters::Website::SephoraSpain,
                parameters::Website::Maquillalia,
            ],
            sort_by: parameters::SortingType::Similarity,
        };
        let parameters_processor = ParametersProcessor::new(args);
        let scraper_handler = ScraperHandler::new(parameters_processor);
        let results_by_website = scraper_handler.get_results();
        assert_eq!(results_by_website.len(), 2);
    }

    /// Tests a search for a product in all websites.
    #[test]
    #[ignore]
    fn get_results_all_websites() {
        let args = Args {
            product: String::from("labial"),
            max_results: 50,
            min_similarity: 0.0,
            websites: vec![parameters::Website::All],
            sort_by: parameters::SortingType::Similarity,
        };
        let parameters_processor = ParametersProcessor::new(args);
        let scraper_handler = ScraperHandler::new(parameters_processor);
        let _results_by_website = scraper_handler.get_results();
    }
}
