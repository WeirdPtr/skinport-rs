use skinport::{
    client::Client,
    util::{currency::Currency, game::Game},
};

#[tokio::main]
async fn main() {
    let client = Client::default();

    let items = client
        .get_sales_out_of_stock(Some(Game::CSGO), Some(Currency::EUR))
        .await;

    println!("{items:#?}");
}
