//! This file handles the visualization in the terminal.

use std::collections::HashMap;

use scrapped_webs::product::Product;

use crate::parameters;

type ResultsByWebsite = HashMap<parameters::Website, Vec<Product>>;

/// Prints the formatted output in the terminal
/// # Example
/// 95%. Labial Rare Beauty - 9.99 - 7.5⭐ - www.sephora.es ///
///     - ✔️ Tone1 ̶- 9̶.̶9̶9̶  4.99(50%) - 9.5⭐               ///
///     - ❌ Tone2 - 9.99                                  ///
/// 72%. Colorete Sephora - 9.99 - 7.5⭐ - www.sephora.es ///
///     - ✔️ Tone1 ̶- 9̶.̶9̶9̶  4.99(50%) - 9.5⭐             ///
pub fn print(results_by_website: &ResultsByWebsite) {
    // Right now we are not using the website to print since the results are already sorted and filtered.
    for product in results_by_website.values().flatten() {
        println!();
        println!("{}", product.terminal_format());
        if let Some(tones) = product.tones.as_ref() {
            for tone in tones {
                println!("{}", tone.terminal_format());
            }
        };
    }
}

#[cfg(test)]
mod tests {
    use scrapped_webs::product::Tone;

    use crate::parameters::Website;

    use super::*;

    /// Tests the print function.
    #[test]
    fn print_function_happy_path() {
        let tone: Tone = Tone::new(
            Some(String::from("Tone 1")),
            Some(50.99),
            None,
            true,
            None,
            None,
        );
        let tone_on_sale: Tone = Tone::new(
            Some(String::from("Tone 1")),
            Some(50.99),
            Some(20.0),
            true,
            None,
            None,
        );
        let product: Product = Product::new(
            String::from("Product 1"),
            Some(String::from("Brand")),
            String::from("http://www.test.com"),
            Some(10.0),
            Some(5.0),
            Some(vec![tone, tone_on_sale]),
            Some(0.95421),
            0.92,
            true,
        );

        let mut results_by_websites: ResultsByWebsite = ResultsByWebsite::new();
        results_by_websites.insert(Website::SephoraSpain, vec![product]);
        print(&results_by_websites);
    }
}
