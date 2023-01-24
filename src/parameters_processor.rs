//! Utilities to handle the command send via command line or translated via server petition

use std::collections::HashMap;

use crate::parameters::{self, Args};
use clap::Parser;
use scrapped_webs::{
    configuration::{self, Configuration},
    product::Product,
    scrappable::Scrappable,
    webs::{maquillalia::Maquillalia, sephora::spain::SephoraSpain},
};

/// Struct to store the parameters set by the user
#[derive(Debug)]
pub struct ParametersProcessor {
    configuration: Configuration,
    websites: Vec<parameters::Website>,
    product: String,
    sorting_type: parameters::SortingType,
}

impl ParametersProcessor {
    /// Creates a new ParametersProcessor.
    /// # Arguments
    /// * `args` - The arguments sent by the user.
    /// # Returns
    /// A new ParametersProcessor.
    pub fn new(args: Args) -> Self {
        let mut min_similarity = args.min_similarity;
        if min_similarity > 1.0 {
            min_similarity = 1.0;
        }
        let mut max_results = args.max_results;
        if max_results > configuration::MAX_RESULTS {
            max_results = configuration::MAX_RESULTS;
        }
        let conf: Configuration = Configuration::new(min_similarity, max_results);
        Self {
            configuration: conf,
            websites: args.websites,
            product: args.product,
            sorting_type: args.sort_by,
        }
    }

    /// Returns the configuration for the search.
    pub fn configuration(&self) -> &Configuration {
        &self.configuration
    }

    /// Returns the product to search.
    pub fn product(&self) -> &String {
        &self.product
    }

    /// Returns the websites to search.
    pub fn websites(&self) -> &Vec<parameters::Website> {
        &self.websites
    }

    /// Returns the sorting type.
    pub fn sorting_type(&self) -> &parameters::SortingType {
        &self.sorting_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Setups the test.
    fn tear_up(
        product: String,
        max_results: usize,
        min_similarity: f32,
        websites: Vec<parameters::Website>,
        sort_by: parameters::SortingType,
    ) -> ParametersProcessor {
        let args = Args {
            product,
            max_results,
            min_similarity,
            websites,
            sort_by,
        };
        ParametersProcessor::new(args)
    }

    /// Tests the new method with a happy path.
    #[test]
    fn new_happy_path() {
        let parameters_processor = tear_up(
            String::from("Pintalabios"),
            15,
            0.0,
            vec![parameters::Website::SephoraSpain],
            parameters::SortingType::Similarity,
        );
        assert_eq!(parameters_processor.product(), "Pintalabios");
        assert_eq!(parameters_processor.configuration().max_results(), 15);
        assert_eq!(parameters_processor.configuration().min_similarity(), 0.0);
        assert_eq!(
            parameters_processor.websites(),
            &vec![parameters::Website::SephoraSpain]
        );
    }

    /// Tests the new method with a max results greater than the max allowed.
    #[test]
    fn new_max_results_greater_than_max_allowed() {
        let parameters_processor = tear_up(
            String::from("Pintalabios"),
            1000,
            0.0,
            vec![parameters::Website::SephoraSpain],
            parameters::SortingType::Similarity,
        );
        assert_eq!(
            parameters_processor.configuration().max_results(),
            configuration::MAX_RESULTS
        );
    }

    /// Tests the new method with a min similarity greater than 1.
    #[test]
    fn new_min_similarity_greater_than_1() {
        let parameters_processor = tear_up(
            String::from("Pintalabios"),
            15,
            1.1,
            vec![parameters::Website::SephoraSpain],
            parameters::SortingType::Similarity,
        );
        assert_eq!(parameters_processor.configuration().min_similarity(), 1.0);
    }

    /// Tests the configuration method.
    #[test]
    fn configuration() {
        let max_results = 15;
        let min_similarity = 0.0;
        let parameters_processor = tear_up(
            String::from("Pintalabios"),
            max_results,
            min_similarity,
            vec![parameters::Website::SephoraSpain],
            parameters::SortingType::Similarity,
        );
        assert_eq!(
            parameters_processor.configuration().max_results(),
            max_results
        );
        assert_eq!(
            parameters_processor.configuration().min_similarity(),
            min_similarity
        );
    }

    /// Tests the websites method.
    #[test]
    fn websites() {
        let websites = vec![parameters::Website::SephoraSpain];
        let parameters_processor = tear_up(
            String::from("Pintalabios"),
            15,
            0.0,
            websites.clone(),
            parameters::SortingType::Similarity,
        );
        assert_eq!(
            *parameters_processor.websites().first().unwrap(),
            *websites.first().unwrap()
        );
    }

    /// Tests the product method.
    #[test]
    fn product() {
        let product = String::from("Pintalabios");
        let parameters_processor = tear_up(
            product.clone(),
            15,
            0.0,
            vec![parameters::Website::SephoraSpain],
            parameters::SortingType::Similarity,
        );
        assert_eq!(*parameters_processor.product(), product);
    }
}
