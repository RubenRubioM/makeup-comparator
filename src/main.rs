use std::collections::HashMap;
use std::env;

use scrapped_webs::product::Product;
use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::sephora::sephora_spain::SephoraSpain;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let mut products_by_shop = HashMap::<&str, Vec<Product>>::new();
        match SephoraSpain::look_for_products(args.get(1).unwrap().to_string()) {
            Ok(products) => {
                products_by_shop.insert("SephoraSpain", products);
            }
            Err(search_error) => {
                eprintln!("{search_error}");
            }
        };

        println!("{:#?}", products_by_shop);
        println!("Makeup comparator!");
    } else {
        eprintln!("Provide a product to search as an input parameter.");
    }
}
