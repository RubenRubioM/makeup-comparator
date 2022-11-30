#[cfg(test)]
mod product {
    use scrapped_webs::product::*;

    /// Tests the Tone::new function.
    #[test]
    fn tone_instantiation_getters_and_setters() {
        let name: String = String::from("Tone1");
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let available: bool = true;
        let url: Option<String> = Some(String::from("www.tone.es"));
        let rating: Option<f32> = Some(5.0);
        let mut tone: Tone = Tone::new(
            name.clone(),
            price_standard,
            price_sales,
            available,
            url.clone(),
            rating.clone(),
        );

        // Getters
        assert_eq!(*tone.name(), name);
        assert_eq!(tone.price_standard(), price_standard);
        assert_eq!(tone.price_sales().unwrap(), price_sales.unwrap());
        assert_eq!(tone.available(), available);
        assert_eq!(tone.url(), url);
        assert_eq!(tone.rating(), rating);

        // Setters
        let set_name = String::from("Tone2");
        let set_price_standard: f32 = 100.0;
        let set_price_sales: Option<f32> = Some(50.0);
        let set_available: bool = false;
        let set_url: Option<String> = Some(String::from("www.tone2.es"));
        let set_rating: Option<f32> = Some(4.0);

        tone.set_name(set_name.clone());
        tone.set_price_standard(set_price_standard);
        tone.set_price_sales(set_price_sales.clone());
        tone.set_available(set_available);
        tone.set_url(set_url.clone());
        tone.set_rating(set_rating.clone());

        println!("Testing Debug trait implementation for Tone: {:?}", tone);
        println!("Testing Display trait implementation for Tone: {}", tone);
    }

    /// Tests the Product::new function.
    #[test]
    fn product_instantiation_getters_and_setters() {
        let name: String = String::from("Test");
        let brand: String = String::from("Test Brand");
        let link: String = String::from("http://test.es");
        let tone_name: String = String::from("Tone 1");
        let price_standard: f32 = 50.0;
        let price_sales: Option<f32> = Some(25.0);
        let available: bool = true;
        let url: Option<String> = Some(String::from("www.tone.es"));
        let tone_rating: Option<f32> = Some(5.0);
        let tones: Option<Vec<Tone>> = Some(vec![Tone::new(
            tone_name.clone(),
            price_standard,
            price_sales,
            available,
            url.clone(),
            tone_rating.clone(),
        )]);
        let rating: Option<f32> = Some(4.5);
        let similarity: f32 = 0.86;
        let mut product: Product = Product::new(
            name.clone(),
            brand.clone(),
            link.clone(),
            price_standard,
            price_sales,
            tones,
            rating,
            similarity,
            available,
        );

        // Getters
        assert_eq!(*product.name(), name);
        assert_eq!(*product.brand(), brand);
        assert_eq!(*product.link(), link);
        assert_eq!(product.price_standard(), price_standard);
        assert_eq!(product.price_sales().unwrap(), price_sales.unwrap());
        assert_eq!(product.rating().unwrap(), rating.unwrap());
        assert_eq!(product.similarity(), similarity);
        assert_eq!(product.available(), available);

        assert_eq!(*product.tones().unwrap().first().unwrap().name(), tone_name);
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_standard(),
            price_standard
        );
        assert_eq!(
            product.tones().unwrap().first().unwrap().price_sales(),
            price_sales
        );
        assert_eq!(
            product.tones().unwrap().first().unwrap().available(),
            available
        );
        assert_eq!(product.tones().unwrap().first().unwrap().url(), url);
        assert_eq!(
            product.tones().unwrap().first().unwrap().rating(),
            tone_rating
        );

        // Setters
        let set_name: String = String::from("Test 2");
        let set_brand: String = String::from("Test Brand 2");
        let set_link: String = String::from("http://test2.es");
        let set_tone_name: String = String::from("Tone 2");
        let set_price_standard: f32 = 100.0;
        let set_price_sales: Option<f32> = Some(50.0);
        let set_available: bool = false;
        let set_url: Option<String> = Some(String::from("www.tone2.es"));
        let set_tone_rating: Option<f32> = Some(4.0);
        let set_tones: Option<Vec<Tone>> = Some(vec![Tone::new(
            set_tone_name.clone(),
            set_price_standard,
            set_price_sales,
            set_available,
            set_url.clone(),
            set_tone_rating.clone(),
        )]);
        let set_rating: Option<f32> = Some(4.0);
        let set_similarity: f32 = 0.75;

        product.set_name(set_name.clone());
        product.set_brand(set_brand.clone());
        product.set_link(set_link.clone());
        product.set_price_standard(set_price_standard);
        product.set_price_sales(set_price_sales.clone());
        product.set_tones(set_tones.clone());
        product.set_rating(set_rating.clone());
        product.set_similarity(set_similarity);
        product.set_available(set_available);

        println!(
            "Testing Debug trait implementation for Product: {:?}",
            product
        );
        println!(
            "Testing Display trait implementation for Product: {}",
            product
        );
    }
}
