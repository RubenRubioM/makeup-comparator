use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::sephora::sephora_spain::SephoraSpain;

fn main() {
    let _s = SephoraSpain::look_for_products("Kind Words").unwrap();

    println!("Makeup comparator!");
}
