use std::fmt::{Display, Error, Formatter};

use clap::{clap_derive::ArgEnum, Parser};

#[derive(ArgEnum, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Website {
    /// All the websites
    All,
    /// www.sephora.es
    SephoraSpain,
    /// www.maquillalia.com
    Maquillalia,
}

#[derive(ArgEnum, Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum SortingType {
    /// Name of the product
    Name,
    /// Price of the product
    Price,
    /// Similarity
    Similarity,
    /// Brand of the product
    Brand,
    /// Rating of the product
    Rating,
}

impl Display for SortingType {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            SortingType::Name => write!(f, "name"),
            SortingType::Price => write!(f, "price"),
            SortingType::Similarity => write!(f, "similarity"),
            SortingType::Brand => write!(f, "brand"),
            SortingType::Rating => write!(f, "rating"),
        }
    }
}

/// A simple command line finder and comparator for makeups websites
#[derive(Parser, Debug, Clone)]
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
    /// Sorting criteria
    #[clap(long, value_parser, default_value_t = SortingType::Similarity)]
    pub sort_by: SortingType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_args() {
        let args = Args {
            product: String::from("Pintalabios"),
            max_results: 15,
            min_similarity: 0.0,
            websites: vec![Website::All],
            sort_by: SortingType::Price,
        };
        assert_eq!(args.product, "Pintalabios");
        assert_eq!(args.max_results, 15);
        assert_eq!(args.min_similarity, 0.0);
        assert_eq!(args.websites, vec![Website::All]);
    }
}
