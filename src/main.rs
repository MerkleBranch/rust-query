use structopt::StructOpt;
use serde_derive::{Deserialize};
use reqwest::Url;
use exitfailure::ExitFailure;

#[derive(StructOpt)]
struct Cli {
    public_address: String,
}

#[derive(Deserialize)]
struct WalletQuery {
    data: Data,
}
#[derive(Deserialize)]
struct Data {
    address: String,
    updated_at: String,
    quote_currency: String,
    items: Vec<Items>,
}
#[derive(Deserialize)]
struct Items {
    contract_name: String,
    balance: String,
    quote: f32,
}

impl WalletQuery {
    async fn get(public_address: &String) -> Result<Self, ExitFailure> {
        let api_key = "";
        let url = format!(
            "https://api.covalenthq.com/v1/1/address/{}/balances_v2/?&key={}",
            public_address, api_key
        );
        let url = Url::parse(&*url)?;
        let resp = reqwest::get(url).await?.json::<WalletQuery>().await?;
        Ok(resp)
    }
}

#[tokio::main]
async fn main() -> Result<(), ExitFailure> {
    let args = Cli::from_args();
    let response: WalletQuery = WalletQuery::get(&args.public_address).await?;

    println!("this wallet: {}", response.data.address);
    println!("last update: {} ", response.data.updated_at);
    println!("measured in: {} \n", response.data.quote_currency);
    println!("first three coins are: \n");
    println!("name: {}", response.data.items[0].contract_name);
    println!("number of coins: {}", response.data.items[0].balance);
    println!("total value: {} \n", response.data.items[0].quote);
    println!("name: {}", response.data.items[1].contract_name);
    println!("number of coins: {}", response.data.items[1].balance);
    println!("total value: {} \n", response.data.items[1].quote);
    println!("name: {}", response.data.items[2].contract_name);
    println!("number of coins: {}", response.data.items[2].balance);
    println!("total value: {} \n", response.data.items[2].quote);
    Ok(())
}
