use scrapped_webs::scrappable::Scrappable;
use scrapped_webs::sephora::SephoraSpain;

fn main() {
    let _s = SephoraSpain::search_product("Willyrex").unwrap();

    println!("Makeup comparator!");
}
