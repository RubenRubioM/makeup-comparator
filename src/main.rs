#![allow(unused_imports)]
use std::collections::HashMap;

mod parameters;
mod parameters_processor;
mod scraper_handler;

use parameters_processor::ParametersProcessor;
use scraper_handler::ScraperHandler;
use scrapped_webs::configuration::Configuration;
use scrapped_webs::product::Product;
use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::webs::maquillalia::Maquillalia;
use scrapped_webs::webs::sephora::spain::SephoraSpain;

use clap::Parser;
use parameters::Args;

use clap::Arg;

fn main() {
    let parameters_processor = ParametersProcessor::new(Args::parse());
    let scraper_handler = ScraperHandler::new(parameters_processor);
    let products_by_shop = scraper_handler.get_results();
    println!("{:#?}", products_by_shop);
    println!("Makeup comparator!");
}
