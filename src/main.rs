#![allow(unused_imports)]
use std::collections::HashMap;

mod parameters;
mod parameters_processor;
mod scraper_handler;
mod terminal_visualizer;

use parameters_processor::ParametersProcessor;
use scraper_handler::ScraperHandler;
use scrapped_webs::configuration::Configuration;
use scrapped_webs::product::Product;
use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::webs::maquillalia::Maquillalia;
use scrapped_webs::webs::sephora::spain::SephoraSpain;

use clap::Parser;
use parameters::Args;
use scrapped_webs::product::Tone;

use clap::Arg;

fn main() {
    let parameters_processor = ParametersProcessor::new(Args::parse());
    let scraper_handler = ScraperHandler::new(parameters_processor);
    let results_by_website = scraper_handler.get_results();
    terminal_visualizer::print(&results_by_website);
    println!("{:#?}", results_by_website);
    println!("Makeup comparator!");
}
