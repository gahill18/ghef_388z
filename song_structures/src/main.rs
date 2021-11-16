use genius_rs::Genius;

async fn scrape_artist(artist: &str) -> &str{
    let token = "tu4mFXq-j8GlG9mqZQlEBb0yeekm_zC5A3-mt2RxYVF3qTAPrLybFl_ykJ7Fk_E5-dyQ7bMCXHNSbaYjCOQ47g";
    let genius = Genius::new(token.to_string());
    let response = genius.search(artist).await.unwrap();
    println!("{}", response[0].result.full_title);
    response
}

fn main() {
    let results = scrape_artist("Ariana Grande");

}
