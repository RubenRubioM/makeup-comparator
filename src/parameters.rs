use clap::{clap_derive::ArgEnum, Parser};

#[derive(ArgEnum, Clone, Debug)]
pub enum Website {
    /// All the websites
    All,
    /// www.sephora.es
    SephoraSpain,
    /// www.maquillalia.com
    Maquillalia,
}

/// A simple command line finder and comparator for makeups websites
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the product to search and compare
    #[clap(short, long, value_parser)]
    pub product: String,
    /// Maximum number of results
    #[clap(long, value_parser, default_value_t = 50)]
    pub max_results: usize,
    /// Minimum similarity threshold
    #[clap(long, value_parser, default_value_t = 0.0)]
    pub min_similarity: f32,
    /// Websites to search
    #[clap(long, value_parser)]
    pub websites: Vec<Website>,
}
