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
    let webs = args.websites;
    let mut products_by_shop = HashMap::<&str, Vec<Product>>::new();

    for web in webs {
        match web {
            parameters::Website::SephoraSpain => {
                let sephora_spain = SephoraSpain::new(&conf);
                let products = sephora_spain
                    .look_for_products(args.product.clone())
                    .unwrap();
                products_by_shop.insert("SephoraSpain", products);
            }
            parameters::Website::Maquillalia => {
                let maquillalia = Maquillalia::new(&conf);
                let products = maquillalia.look_for_products(args.product.clone()).unwrap();
                products_by_shop.insert("Maquillalia", products);
            }
            parameters::Website::All => todo!(),
        }
    }
    println!("{:#?}", products_by_shop);
    println!("Makeup comparator!");
}
