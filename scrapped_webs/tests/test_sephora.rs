#[allow(non_snake_case)]
#[cfg(test)]
mod sephora_spain {
    use scrapped_webs::scrappable::*;
    use scrapped_webs::sephora::sephora_spain::*;

    /// Tests if the SephoraSpain::look_for_prducts(name) works when we search for a product and get redirected.
    /// If at some point fails, might be because the webpage changed or the product for search is not avaliable anymore.
    #[test]
    #[ignore]
    fn WhenCallingLookForProductsWithUrlRedirection_ThenSuccess() {
        let products = SephoraSpain::look_for_products(String::from("ISDIN protector labial")).unwrap();
        assert_eq!(products.len(), 1);

        // TODO: Assert if the values are returned properly.
        let product = products.get(0).unwrap();
        assert_eq!(*product.name(), "Protector labial spf50+ - Protector labial");
        assert_eq!(*product.brand(), "ISDIN");
        assert_eq!(*product.link(), "https://www.sephora.es/p/protector-labial-spf50---protector-labial-469417.html");
        assert_eq!(product.price_standard(), 0.0);
        assert_eq!(product.price_sales(), None);
        assert_eq!(product.rating(), None);
        assert_eq!(product.similarity(), 1.0);
        assert_eq!(product.tones().unwrap().len(), 1);
        assert_eq!(*product.tones().unwrap().first().unwrap().name(), "4 g");
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_standard(),
            7.99
        );
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_sales(),
            None
        );
    }

    /// Tests if the SephoraSpain::look_for_prducts(name) works.
    /// If at some point fails, might be because the webpage changed, the product for search is not avaliable anymore or there are new entries for this search.
    #[test]
    #[ignore]
    fn WhenCallingLookForProductsWithSearchResults_ThenSuccess() {
        let products =
            SephoraSpain::look_for_products(String::from("RARE BEAUTY Kind Words")).unwrap();
        assert_eq!(products.len(), 2);

    }

    /// Tests if the SephoraSpain::look_for_prducts(name) return errors properly.
    #[test]
    #[ignore]
    fn WhenCallingLookForProductsWithoutResults_ThenReturnErrors() {
        match SephoraSpain::look_for_products(String::from("Taemin")) {
            Ok(_) => panic!("We should not find any results"),
            Err(search_error) => match search_error {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughtSimilarity => panic!("{}", search_error),
                SearchError::ForbiddenWord(_) => panic!("{}", search_error),
                SearchError::NotFound => assert!(true),
            },
        }

        match SephoraSpain::look_for_products(String::from("Sephora")) {
            Ok(_) => panic!("We should not find any results"),
            Err(search_error) => match search_error {
                SearchError::Timeout => panic!("{}", search_error),
                SearchError::NotEnoughtSimilarity => panic!("{}", search_error),
                SearchError::ForbiddenWord(_) => assert!(true),
                SearchError::NotFound => panic!("{}", search_error),
            },
        }
    }
}