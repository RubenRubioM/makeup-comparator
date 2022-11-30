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

    /// Tests the parsing between the full name in Maquillalia and a custom version without the final Tone specification.
    #[test]
    fn get_product_without_tone() {
        let full_name: String =
            String::from("Maybelline - Labial líquido SuperStay Vinyl Ink - 35: Cheeky");
        let product_name: String = Maquillalia::get_name_without_tone(&full_name);
        assert_eq!(
            product_name,
            "Maybelline - Labial líquido SuperStay Vinyl Ink"
        );
    }

    /// Tests the parsing between the full name in Maquillalia and a custom version with only the final tone name.
    #[test]
    fn get_tone_name() {
        let full_name: String =
            String::from("Maybelline - Labial líquido SuperStay Vinyl Ink - 35: Cheeky");
        let tone_name: String = Maquillalia::get_tone_name(&full_name);
        assert_eq!(tone_name, " 35: Cheeky");

        //TODO: Support for tones with trailing dashes.
        // let full_name: String = String::from("Maybelline - Labial líquido SuperStay Vinyl Ink - 35: Cheeky - Extra long - dashes - - - - -");
        // let tone_name: String = Maquillalia::get_tone_name(&full_name);
        // assert_eq!(tone_name, " 35: Cheeky - Extra long - dashes - - - - -");
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
        const MAX_RESULTS: usize = 3;
        let conf: Configuration = Configuration::new(0.0, MAX_RESULTS);
        let products = Maquillalia::new(&conf)
            .look_for_products(String::from("Labial"))
            .unwrap();
        assert_eq!(products.len(), MAX_RESULTS);
    }

    /// Tests a search with no results.
    #[test]
    fn search_with_no_results() {
        let conf: Configuration = Configuration::new(0.95, usize::MAX);
        match Maquillalia::new(&conf).look_for_products(String::from("taemin")) {
            Ok(_) => panic!("We should not retrieve any results in this search"),
            Err(search_error) => match search_error {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughSimilarity => panic!("{}", search_error),
                SearchError::NotFound => assert!(true),
            },
        };
        match Maquillalia::new(&conf).look_for_products(String::from("iluminador facial")) {
            Ok(_) => panic!("We should not retrieve any results in this search"),
            Err(search_error) => match search_error {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughSimilarity => assert!(true),
                SearchError::NotFound => panic!("{}", search_error),
            },
        };
    }

    /// Tests a search for a product with different tones available.
    #[test]
    fn search_product_with_tones() {
        let conf: Configuration = Configuration::new(0.0, 1);
        let products = Maquillalia::new(&conf)
            .look_for_products(String::from("Milani - Labial Líquido Amore Mettallics"))
            .unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(
            products.first().unwrap().name(),
            " Labial Líquido Amore Mettallics"
        );
        assert_eq!(products.first().unwrap().brand(), "Milani ");
        assert_eq!(products.first().unwrap().price_standard(), 0.0);
        assert_eq!(products.first().unwrap().price_sales(), None);
        assert_eq!(products.first().unwrap().rating(), None);
        assert_eq!(products.first().unwrap().tones().unwrap().len(), 4);
    }

    #[test]
    fn search_product_without_tones() {
        let conf: Configuration = Configuration::new(0.0, 1);
        let products = Maquillalia::new(&conf)
            .look_for_products(String::from("Agrado - Bruma facial solar SPF50+"))
            .unwrap();
        assert_eq!(products.len(), 1);
        assert_eq!(
            products.first().unwrap().name(),
            " Bruma facial solar SPF50+"
        );
        assert_eq!(products.first().unwrap().brand(), "Agrado ");
        assert_eq!(products.first().unwrap().tones().is_none(), true);
    }
}
