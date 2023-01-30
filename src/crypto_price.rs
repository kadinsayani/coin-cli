use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::thread::sleep;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinbasePrice {
    pub data: CoinPrice,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoinPrice {
    pub base: String,
    pub currency: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Time {
    pub data: PriceTime,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PriceTime {
    pub iso: String,
}

#[derive(Parser, Debug)]
#[clap(
    author,
    version = "0.1.8",
    about = "cryptop - Command Line Interface for getting cryptocurrency prices and information." // description
)]
struct Cli {
    #[clap(short, long, default_value = "BTC")]
    currency: String, // -c, --currency
    #[clap(short, long, default_value = "USD")]
    rates: String, // -r, --rates
}

pub fn crypto_price() {
    let args = Cli::parse(); // Parse the command line arguments

    let currency = &args.currency; // Get the currency
    let rates = &args.rates; // Get the rates

    let spot_price = get_coin_price("spot".to_string(), currency.to_string(), rates.to_string());
    println!(
        "{}-{} Current Price: {:?}", // Print the current price
        currency.to_string(),
        rates.to_string(),
        spot_price.unwrap()
    );

    let price_time = get_time();
    println!("Time: {}", price_time.unwrap())
}

#[tokio::main]
async fn get_coin_price(
    request_type: String,
    request_currency: String,
    request_rates: String,
) -> Result<std::string::String, Box<dyn std::error::Error>> {
    let request_url_spot = format!("https://api.coinbase.com/v2/prices/{currency}-{rates}/{type}",
        currency = request_currency,
        rates = request_rates,
        type = request_type);

    let pb = ProgressBar::new(100);

    match ProgressStyle::default_spinner().template("{spinner} {msg}") {
        Ok(style) => {
            pb.set_style(style);
            pb.set_message("Retrieving current price...");
            pb.enable_steady_tick(Duration::from_millis(100));
        }
        Err(error) => println!("Error: {}", error),
    }

    let client = Client::new();
    let resp_price = client
        .get(&request_url_spot)
        .send()
        .await?
        .json::<CoinbasePrice>()
        .await?;

    sleep(Duration::from_secs(1));

    pb.finish_with_message("Current price retrieved");

    Ok(resp_price.data.amount)
}

#[tokio::main]
async fn get_time() -> Result<std::string::String, Box<dyn std::error::Error>> {
    let request_url_time = format!("https://api.coinbase.com/v2/time");
    let client = Client::new();
    let resp_time = client
        .get(&request_url_time)
        .send()
        .await?
        .json::<Time>()
        .await?;

    Ok(resp_time.data.iso)
}
