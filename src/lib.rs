use reqwest::Error;
use serde::{Deserialize};
//use serde_json::Result;

#[derive(Debug)]
pub struct Account{
    api_key: String,
    trading_url: String
}

impl Account{
    pub fn new(api_key: String, trading_url: String) -> Account{
        Account{api_key, trading_url}
    }

    pub async fn account_information(&self) -> Result<AccountInformation, reqwest::Error>{
        let url = self.trading_url.clone() + "account/";
        //let text: serde_json::AccountInformation = reqwest::blocking::get(url).send().json()?;


        let acc_info: AccountInformation = reqwest::Client::new().get(url).header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", "Bearer ".to_owned() + &self.api_key)
        .send()
        .await?
        .json()
        .await?;
        println!("{:?}",acc_info);
        Ok(acc_info)
    }
}

#[derive(Deserialize, Debug)]
pub struct AccountInformation{
    time: String,
    mode: String,
    status: String,
    results: Results
}


#[derive( Deserialize, Debug)]
pub struct Results{
    created_at: String,
    account_id: String,
    firstname: String,
    lastname: Option<String>,
    email: String,
    phone: Option<String>,
    address: Option<String>,
    billing_address: Option<String>,
    billing_email: Option<String>,
    billing_name: Option<String>,
    billing_vat: Option<String>,
    mode: String,
    deposit_id: Option<String>,
    client_id: Option<String>,
    account_number: Option<String>,
    iban_brokerage: Option<String>,
    iban_origin: Option<String>,
    bank_name_origin: Option<String>,
    balance: i32, //(Your end-of-day balance from the day before) + (amount_sold_intraday) - (amount_bought_intraday) - (amount_open_withdrawals).
    cash_to_invest: i32, //(balance) - (amount_open_orders)
    cash_to_withdraw: i32, // (Your end-of-day balance from the day before) - (amount_bought_intraday) - (amount_open_withdrawals) - (amount_open_orders).
    amount_bought_intraday: i32,
    amount_sold_intraday: i32,
    amount_open_orders: i32,
    amount_open_withdrawals: i32,
    amount_estimate_taxes: i32,
    approved_at: Option<String>, //Time
    trading_plan: String,
    data_plan:String,
    tax_allowance: Option<i32>,
    tax_allowance_start: Option<String>,
    tax_allowance_end: Option<String>
}