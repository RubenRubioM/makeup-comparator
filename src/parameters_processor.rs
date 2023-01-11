//! Utilities to handle the command send via command line or translated via server petition

use std::collections::HashMap;

use crate::parameters::{self, Args};
use clap::Parser;
use scrapped_webs::{
    configuration::Configuration,
    product::Product,
    scrappable::Scrappable,
    webs::{maquillalia::Maquillalia, sephora::spain::SephoraSpain},
};

pub fn get_results(args: Args) -> HashMap<parameters::Website, Vec<Product>> {
    let conf: Configuration = Configuration::new(args.min_similarity, args.max_results);
    let mut products_by_shop = HashMap::<parameters::Website, Vec<Product>>::new();
    let webs = args.websites;

    for web in webs {
        match web {
            parameters::Website::SephoraSpain => {
                let sephora_spain = SephoraSpain::new(&conf);
                let products = sephora_spain
                    .look_for_products(args.product.clone())
                    .unwrap();
                products_by_shop.insert(parameters::Website::SephoraSpain, products);
            }
            parameters::Website::Maquillalia => {
                let maquillalia = Maquillalia::new(&conf);
                let products = maquillalia.look_for_products(args.product.clone()).unwrap();
                products_by_shop.insert(parameters::Website::Maquillalia, products);
            }
            parameters::Website::All => todo!(),
        }
    }
    products_by_shop
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests a search for a product in two websites.
    #[test]
    fn test_get_results() {
        let args = Args {
            product: "pintalabios".to_string(),
            max_results: 50,
            min_similarity: 0.0,
            websites: vec![
                parameters::Website::SephoraSpain,
                parameters::Website::Maquillalia,
            ],
        };
        let products_by_shop = get_results(args);
        assert_eq!(products_by_shop.len(), 2);
    }

    /// Tests a search for a product in all websites.
    /// TODO: Implement the Website::All
    #[test]
    #[should_panic]
    fn test_get_results_all_websites() {
        let args = Args {
            product: "pintalabios".to_string(),
            max_results: 50,
            min_similarity: 0.0,
            websites: vec![parameters::Website::All],
        };
        get_results(args);
    }
}
