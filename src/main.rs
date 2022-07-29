use std::collections::HashMap;

use scrapped_webs::product::Product;
use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::sephora::sephora_spain::SephoraSpain;

fn main() {
    let mut products_by_shop = HashMap::<&str, Vec<Product>>::new();
    match SephoraSpain::look_for_products("FauxFilter Foundation Stick Base De Maquillaje En Stick")
    {
        Ok(products) => {
            products_by_shop.insert("SephoraSpain", products);
        }
        Err(search_error) => {
            eprintln!("{search_error}");
        }
    };

    println!("{:#?}", products_by_shop);
    println!("Makeup comparator!");
}
