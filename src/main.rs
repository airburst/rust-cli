// use clap::Parser;
// use anyhow::{anyhow, Result};
use reqwest;
use serde::Deserialize;

// /// Simple program to greet a person
// #[derive(Parser, Debug)]
// #[command(author, version, about, long_about = None)]
// struct Args {
//     /// Your Last.fm API Key
//     #[arg(short = 'k', long)]
//     api_key: String,

//     /// Your Last.fm Username
//     #[arg(short, long)]
//     username: String,

//     /// The limit of Artists
//     #[arg(short, long)]
//     limit: u16,

//     /// The lookback period
//     #[arg(short, long, default_value = "7day")]
//     period: String,
// }

#[derive(Debug, Deserialize)]
struct Beer {
    id: usize,
    name: String,
    abv: f64,
    food_pairing: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    // let args = Args::parse();

    // Fetch data from API and cast to type
    let beers: Vec<Beer> = reqwest::Client::new()
        .get("https://api.punkapi.com/v2/beers")
        .send()
        .await?
        .json()
        .await?;

    // Format json object into string
    for beer in beers {
        println!(
            "{}) {} ({}%) ... [{}]",
            beer.id,
            beer.name,
            beer.abv,
            beer.food_pairing.join(", ")
        );
    }
    // } else {
    //     return Err(anyhow!("Unable to convert response to JSON."));
    // }

    Ok(())
}
