//! This file handles the visualization in the terminal.

use std::collections::HashMap;

use scrapped_webs::product::Product;

use crate::parameters;

type ResultsByWebsite = HashMap<parameters::Website, Vec<Product>>;

/// Prints the formatted output in the terminal
/// # Example
////////////////////////////////////////////////////////////////////////////////////////////////
/// Labial
pub fn print(_results_by_website: &ResultsByWebsite) {}
