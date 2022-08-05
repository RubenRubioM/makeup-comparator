//! Module for common data that can be modified.

/// Global configuration for the program.
pub struct Configuration {
    /// The minimum similarity for searching.
    min_similarity: f32,
    /// The max numbers of results.
    max_results: usize,
}

impl Configuration {
    /// Creates a new Configuration
    ///
    /// # Arguments
    /// min_similarity - The minimum similarity from 0 to 1.
    /// max_results - The max number of results to retrieve.
    pub fn new(min_similarity: f32, max_results: usize) -> Self {
        Self {
            min_similarity,
            max_results,
        }
    }
    /// Returns the minimum similarity value.
    pub fn min_similarity(&self) -> f32 {
        self.min_similarity
    }
    /// Returns the max results value.
    pub fn max_results(&self) -> usize {
        self.max_results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn configuration_creation() {
        let configuration = Configuration::new(0.1, 10);
        assert_eq!(configuration.min_similarity(), 0.1);
        assert_eq!(configuration.max_results(), 10);
    }
}
