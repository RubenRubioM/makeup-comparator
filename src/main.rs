#![allow(unused_imports)]
use std::collections::HashMap;

use scrapped_webs::configuration::Configuration;
use scrapped_webs::product::Product;
use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::webs::maquillalia::Maquillalia;
use scrapped_webs::webs::sephora::spain::SephoraSpain;

mod parameters;
mod parameters_processor;
use clap::Parser;
use parameters::Args;

use clap::Arg;

fn main() {
    let products_by_shop = parameters_processor::get_results(Args::parse());
    println!("{:#?}", products_by_shop);
    println!("Makeup comparator!");
}
