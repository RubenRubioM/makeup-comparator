#[allow(non_snake_case)]
#[cfg(test)]
mod product {
    use scrapped_webs::product::*;

    /// Tests the Tone::new function.
    #[test]
    fn WhenCreatingTones_ThenSuccess() {
        let name: String = String::from("Tone1");
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let tone: Tone = Tone::new(name.clone(), price_standard, price_sales);

        assert_eq!(*tone.name(), name);
        assert_eq!(tone.price_standard(), price_standard);
        assert_eq!(tone.price_sales().unwrap(), price_sales.unwrap());
    }

    /// Tests the Product::new function.
    #[test]
    fn WhenCreatingAProduct_ThenSuccess() {
        let name: String = String::from("Test");
        let brand: String = String::from("Test Brand");
        let link: String = String::from("http://test.es");
        let tone_name: String = String::from("Tone 1");
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let tones: Option<Vec<Tone>> = Some(vec![Tone::new(
            tone_name.clone(),
            price_standard,
            price_sales,
        )]);
        let rating: Option<f32> = Some(4.5);
        let similarity: f32 = 0.86;
        let product: Product = Product::new(
            name.clone(),
            brand.clone(),
            link.clone(),
            price_standard,
            price_sales,
            tones,
            rating,
            similarity,
        );

        assert_eq!(*product.name(), name);
        assert_eq!(*product.brand(), brand);
        assert_eq!(*product.link(), link);
        assert_eq!(product.price_standard(), price_standard);
        assert_eq!(product.price_sales().unwrap(), price_sales.unwrap());
        assert_eq!(product.rating().unwrap(), rating.unwrap());
        assert_eq!(product.similarity(), similarity);

        assert_eq!(*product.tones().unwrap().first().unwrap().name(), tone_name);
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_standard(),
            price_standard
        );
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_sales(),
            price_sales
        );
    }
}
