#[allow(non_snake_case)]
#[cfg(test)]
mod product {
    use scrapped_webs::product::Product;

    /// Tests the Product::new function.
    #[test]
    fn WhenCreatingAProduct_ThenSuccess() {
        let name: &str = "Test";
        let link: &str = "http://test.es";
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let tones: Option<Vec<&str>> = Some(vec!["Brown", "Yellow"]);
        let rating: Option<f32> = Some(4.5);
        let similarity: f32 = 0.86;
        let product: Product = Product::new(
            name,
            link,
            price_standard,
            price_sales,
            tones,
            rating,
            similarity,
        );

        assert_eq!(name, product.name());
        assert_eq!(link, product.link());
        assert_eq!(price_standard, product.price_standard());
        assert_eq!(price_sales.unwrap(), product.price_sales().unwrap());
        assert_eq!(rating.unwrap(), product.rating().unwrap());
        assert_eq!(similarity, product.similarity());
    }
}
