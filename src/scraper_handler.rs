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
    pub fn get_results(&self) -> HashMap<parameters::Website, Vec<Product>> {
        let mut products_by_shop = HashMap::<parameters::Website, Vec<Product>>::new();

        for web in self.parameters_processor.websites().iter() {
            match web {
                parameters::Website::SephoraSpain => {
                    let sephora_spain =
                        SephoraSpain::new(self.parameters_processor.configuration());
                    let products = sephora_spain
                        .look_for_products(self.parameters_processor.product().clone())
                        .unwrap();
                    products_by_shop.insert(parameters::Website::SephoraSpain, products);
                }
                parameters::Website::Maquillalia => {
                    let maquillalia = Maquillalia::new(self.parameters_processor.configuration());
                    let products = maquillalia
                        .look_for_products(self.parameters_processor.product().clone())
                        .unwrap();
                    products_by_shop.insert(parameters::Website::Maquillalia, products);
                }
                parameters::Website::All => todo!(),
            }
        }
        products_by_shop
    }
}

#[cfg(test)]
mod tests {
    use crate::scraper_handler;

    use super::*;

    /// Tests the debug trait.
    #[test]
    fn test_debug_trait() {
        let args = Args {
            product: "pintalabios".to_string(),
            max_results: 15,
            min_similarity: 0.0,
            websites: vec![parameters::Website::SephoraSpain],
        };
        let parameters_processor = ParametersProcessor::new(args);
        let scraper_handler = ScraperHandler::new(parameters_processor);
        assert_eq!(
            format!("{:?}", scraper_handler),
            "ScraperHandler { parameters_processor: ParametersProcessor { configuration: Configuration { min_similarity: 0.0, max_results: 15 }, websites: [SephoraSpain], product: \"pintalabios\" } }"
        );
    }

    /// Tests a search for a product in two websites.
    #[test]
    #[ignore]
    fn get_results() {
        let args = Args {
            product: "pintalabios".to_string(),
            max_results: 15,
            min_similarity: 0.0,
            websites: vec![
                parameters::Website::SephoraSpain,
                parameters::Website::Maquillalia,
            ],
        };
        let parameters_processor = ParametersProcessor::new(args);
        let scraper_handler = ScraperHandler::new(parameters_processor);
        let products_by_shop = scraper_handler.get_results();
        assert_eq!(products_by_shop.len(), 2);
    }

    /// Tests a search for a product in all websites.
    /// TODO: Implement the Website::All
    #[test]
    #[should_panic]
    #[ignore]
    fn get_results_all_websites() {
        let args = Args {
            product: "pintalabios".to_string(),
            max_results: 50,
            min_similarity: 0.0,
            websites: vec![parameters::Website::All],
        };
        let parameters_processor = ParametersProcessor::new(args);
        let scraper_handler = ScraperHandler::new(parameters_processor);
        let _products_by_shop = scraper_handler.get_results();
    }
}
