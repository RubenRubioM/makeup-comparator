#[cfg(test)]
mod maquillalia {
    use scrapped_webs::configuration::Configuration;
    use scrapped_webs::scrappable::*;
    use scrapped_webs::webs::maquillalia::Maquillalia;

    /// Tests the creation for Maquillalia structure.
    #[test]
    fn maquillalia_instantiation() {
        let config: Configuration = Configuration::new(0.50, 100);
        let _ = Maquillalia::new(&config);
        assert!(true);
    }

    /// Tests a search with a few results.
    #[test]
    fn search_with_results() {
        let conf: Configuration = Configuration::new(0.0, usize::MAX);
        let _products = Maquillalia::new(&conf)
            .look_for_products(String::from("super stay vinyl"))
            .unwrap();
        assert_eq!(true, true);
    }

    /// Tests a search with a specific num of results to prove that we are not retrieving more or less than that.
    /// In this test the petition find >1500 results but we have to stop at MAX_RESULTS.
    #[test]
    fn search_with_specific_num_results() {
        const MAX_RESULTS: usize = 21;
        let conf: Configuration = Configuration::new(0.0, MAX_RESULTS);
        let products = Maquillalia::new(&conf)
            .look_for_products(String::from("Labial"))
            .unwrap();
        assert_eq!(products.len(), MAX_RESULTS);
    }

    /// Tests a search with no results.
    #[test]
    fn search_with_no_results() {
        let conf: Configuration = Configuration::new(0.0, usize::MAX);
        match Maquillalia::new(&conf).look_for_products(String::from("taemin")) {
            Ok(_) => panic!("We should not retrieve any results in this search"),
            Err(search_error) => match search_error {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughSimilarity => panic!("{}", search_error),
                SearchError::NotFound => assert!(true),
            },
        }
    }
}
