#[cfg(test)]
mod sephora_spain {
    use scrapped_webs::configuration::Configuration;
    use scrapped_webs::scrappable::*;
    use scrapped_webs::webs::sephora::spain::SephoraSpain;

    /// Tests if SephoraSpain can be created correctly.
    #[test]
    fn sephora_spain_instantiation() {
        let conf: Configuration = Configuration::new(0.0, usize::MAX);
        let _ = SephoraSpain::new(&conf);
        assert!(true);
    }

    /// Tests if the SephoraSpain::look_for_products(name) works when we search for a product and get redirected.
    /// If at some point fails, might be because the webpage changed or the product for search is not available anymore.
    #[test]
    #[ignore]
    fn search_has_url_redirection() {
        let conf: Configuration = Configuration::new(0.0, usize::MAX);
        let sephora_spain = SephoraSpain::new(&conf);
        let products = sephora_spain
            .look_for_products(String::from(
                "SoftSculpt® Shaping Stick - Contorno en barra ",
            ))
            .unwrap();
        assert_eq!(products.len(), 1);

        let product = products.get(0).unwrap();
        assert_eq!(
            *product.name,
            "SoftSculpt® Shaping Stick - Contorno en barra".to_string()
        );
        // assert_eq!(*product.brand(), "ISDIN");
        assert_eq!(
            *product.link,
            "https://www.sephora.es/p/softsculpt-shaping-stick---contorno-en-barra-P10044136.html"
                .to_string()
        );
        assert_eq!(product.price_standard, None);
        assert_eq!(product.price_sales, None);
        // assert_eq!(product.rating(), None); // Not assert by rating since it is changing everyday.
        // assert_eq!(product.similarity(), 1.0);
        assert_eq!(product.tones.as_ref().unwrap().len(), 6);
        assert_eq!(
            product
                .tones
                .as_deref()
                .unwrap()
                .first()
                .unwrap()
                .name
                .as_deref()
                .unwrap(),
            "Light + 10.5g".to_string()
        );
        assert_eq!(
            product
                .tones
                .as_deref()
                .unwrap()
                .first()
                .unwrap()
                .price_standard,
            Some(33.99)
        );
        assert_eq!(
            product
                .tones
                .as_deref()
                .unwrap()
                .first()
                .unwrap()
                .price_sales,
            None
        );
    }

    /// Tests if the SephoraSpain::look_for_products(name) works when the /search? path.
    /// If at some point fails, might be because the webpage changed, the product for search is not available anymore or there are new entries for this search.
    #[test]
    #[ignore]
    fn search_has_results() {
        let conf: Configuration = Configuration::new(0.0, 2);
        let sephora_spain = SephoraSpain::new(&conf);
        let products = sephora_spain
            .look_for_products(String::from("RARE BEAUTY Kind Words"))
            .unwrap();
        assert_eq!(products.len(), 2);
    }

    /// Tests if the SephoraSpain::look_for_products(name) works when the /todos-los-productos/ path.
    /// If at some point fails, might be because the webpage changed, the product for search is not available anymore or there are new entries for this search.
    #[test]
    #[ignore]
    fn search_redirect_to_all_products() {
        let conf: Configuration = Configuration::new(0.0, usize::MAX);
        let sephora_spain = SephoraSpain::new(&conf);
        sephora_spain
            .look_for_products(String::from("Lapiz labial"))
            .unwrap();
        assert!(true);
    }

    /// Tests if the SephoraSpain::look_for_products(name) return errors properly.
    #[test]
    #[ignore]
    fn search_has_no_results() {
        let conf: Configuration = Configuration::new(0.95, usize::MAX);
        let sephora_spain = SephoraSpain::new(&conf);

        match sephora_spain.look_for_products(String::from("Taemin")) {
            Ok(_) => panic!("We should not find any results"),
            Err(search_error) => match search_error.downcast_ref::<SearchError>().unwrap() {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughSimilarity => panic!("{}", search_error),
                SearchError::NotFound => assert!(true),
            },
        }
        match sephora_spain.look_for_products(String::from("iluminador facial")) {
            Ok(_) => panic!("We should not find any results"),
            Err(search_error) => match search_error.downcast_ref::<SearchError>().unwrap() {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughSimilarity => assert!(true),
                SearchError::NotFound => panic!("{}", search_error),
            },
        }
    }
}
