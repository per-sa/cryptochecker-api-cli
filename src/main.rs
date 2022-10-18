use colored::Colorize;
use reqwest;
use serde::Deserialize;
use std::env;
use std::error::Error;

#[derive(Deserialize, Debug)]
struct Response {
    data: Data,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct Data {
    id: String,
    name: String,
    symbol: String,
    priceUsd: String,
}

fn print_info(response: Response) {
    let precision = 3;
    let price = response.data.priceUsd.parse::<f64>().unwrap();

    println!("💬 Name: {}", response.data.name.blue());
    println!("#️⃣  Symbol: {}", response.data.symbol.yellow().bold());
    println!("💰 Price: {:.1$}", price, precision);
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let coin = &args[1];
    let url = format!("https://api.coincap.io/v2/assets/{}", coin);
    let http_response = reqwest::get(url).await?;
    let response = http_response.json::<Response>().await?;

    print_info(response);

    Ok(())
}
