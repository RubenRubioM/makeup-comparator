use crate::{
    product::Product,
    scrappable::{Scrappable, SearchError},
};

use strsim::jaro;

pub struct SephoraSpain;

impl Scrappable for SephoraSpain {
    fn search_product(name: &str) -> Result<Vec<Product>, SearchError> {
        let comparation_name = "willy";
        let similarity = jaro(name, comparation_name);
        println!(
            "Similarity between {} and {} is {}",
            name, comparation_name, similarity
        );
        Ok(Vec::new())
    }
}
