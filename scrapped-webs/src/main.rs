
fn main() {
    let response = reqwest::blocking::get("https://www.sephora.es/p/kind-words---barra-de-labios-mate-P10026821.html")
        .unwrap()
        .text()
        .unwrap();

    let document = scraper::Html::parse_document(&response);

    let title_selector = scraper::Selector::parse("#price-sales").unwrap();

    let titles = document.select(&title_selector).map(|x| x.inner_html());
    println!("{:?}", titles);
    println!("scrapped-webs - main");
}
