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

    /// Tests a search with results.
    #[test]
    fn search_with_results() {
        let conf: Configuration = Configuration::new(0.0, usize::MAX);
        let _products = Maquillalia::new(&conf)
            .look_for_products(String::from("Labial"))
            .unwrap();
        assert_eq!(true, true);
    }
}
