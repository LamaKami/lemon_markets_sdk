use lemon_markets_sdk::Account;

#[tokio::main]
async fn main()  -> Result<(), reqwest::Error>{
    println!("Hello, world!");

    let key = "key".to_owned();
    let acc = Account::new(key, "https://paper-trading.lemon.markets/v1/".to_owned());
    acc.account_information().await?;

    Ok(())
}
