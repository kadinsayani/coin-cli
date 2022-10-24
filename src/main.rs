use clap::Parser;
use std::error::Error;

#[derive(Parser)]
struct Cli {
    command: String,
}

// struct Coin {
//     id: String,
//     symbol: String,
//     name: String,
// }

fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    let command = args.command;
    if command == "list" {
        let response = reqwest::blocking::get("https://api.coingecko.com/api/v3/coins/list")?;
        println!("{}", response.status());
        //     let coins = parse_coins(response)?;
        //     for coin in coins {
        //         println!("{} - {}", coin.id, coin.name);
        //     }
        // }
    }
    Ok(())
}

// fn parse_coins(response: Response) -> Result<Vec<Coin>, Box<dyn Error>> {
//     let body = response.text()?;
//     let coins: Vec<Coin> = serde_json::from_str(&body)?;
//     Ok(coins)
// }
