#![allow(unused_imports)]
use std::collections::HashMap;

use scrapped_webs::configuration::Configuration;
use scrapped_webs::product::Product;
use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::webs::maquillalia::Maquillalia;
use scrapped_webs::webs::sephora::spain::SephoraSpain;

mod parameters;
use clap::Parser;
use parameters::Args;

fn main() {
    let args = Args::parse();
    let conf: Configuration = Configuration::new(args.min_similarity, args.max_results);
    let mut products_by_shop = HashMap::<&str, Vec<Product>>::new();

    // match SephoraSpain::new(&conf).look_for_products(args.product.clone()) {
    //     Ok(products) => {
    //         products_by_shop.insert("SephoraSpain", products);
    //     }
    //     Err(search_error) => {
    //         eprintln!("{search_error}");
    //     }
    // };

    match Maquillalia::new(&conf).look_for_products(args.product) {
        Ok(products) => {
            products_by_shop.insert("Maquillalia", products);
        }
        Err(search_error) => {
            eprintln!("{search_error}");
        }
    };

    println!("{:#?}", products_by_shop);
    println!("Makeup comparator!");
}
