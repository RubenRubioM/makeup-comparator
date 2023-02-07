//! This file handles the visualization in the terminal.

use std::collections::HashMap;

use scrapped_webs::product::Product;

use crate::parameters;

type ResultsByWebsite = HashMap<parameters::Website, Vec<Product>>;

const _ROW: &str = "////////////////////////////////////////////////////////////";
const _COLUMN: &str = "///";

/// Prints the formatted output in the terminal
/// # Example
////////////////////////////////////////////////////////////////
/// 95%. Labial Rare Beauty - 9.99 - 7.5⭐ - www.sephora.es ///
///     - ✔️ Tone1 ̶- 9̶.̶9̶9̶  4.99(50%) - 9.5⭐               ///
///     - ❌ Tone2 - 9.99                                  ///
//////////////////////////////////////////////////////////////
/// 72%. Colorete Sephora - 9.99 - 7.5⭐ - www.sephora.es ///
///     - ✔️ Tone1 ̶- 9̶.̶9̶9̶  4.99(50%) - 9.5⭐             ///
/////////////////////////////////////////////////////////////
pub fn print(_results_by_website: &ResultsByWebsite) {}

#[cfg(test)]
mod tests {
    use super::*;

    /// Tests the print function.
    #[test]
    fn print_function_happy_path() {}
}
