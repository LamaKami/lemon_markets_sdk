use std::collections::HashMap;

use reqwest::{Error, Response};
use serde::{Deserialize, de};
//use serde_json::Result;

#[derive(Debug)]
pub struct LemonMarkets{
    pub account: Account,
    pub orders: Orders,
    pub positons: Positions,
    pub venues: Venues,
    api_key: String,
    trading_url: String,
}

impl LemonMarkets {
    pub fn new(api_key: String, trading_url: String) -> LemonMarkets{
        LemonMarkets{account: Account{trading_sub_url: trading_url.clone()+"account/", api_key: api_key.clone()}
        , api_key: api_key.clone(), trading_url: trading_url.clone(),
        orders: Orders { trading_sub_url: trading_url.clone()+"orders/", api_key: api_key.clone()},
        positons: Positions { trading_sub_url: trading_url.clone()+"positions/", api_key: api_key.clone()},
        venues: Venues { trading_sub_url: "https://data.lemon.markets/v1/venues/".to_owned(), api_key: api_key.clone()}
    }
    }
}


#[derive(Debug)]
pub struct Account{
    trading_sub_url: String,
    api_key: String
}


//TODO Error handeling for every function like token is not correct, etc.
impl Account{
    pub async fn account_information(&self) -> Result<AccountInformation, reqwest::Error>{
        let acc_info: AccountInformation = get_request(&self.trading_sub_url, &self.api_key, vec![]).await?;
        Ok(acc_info)
    }

    pub async fn withdraw_money(&self, amount: i64) -> Result<WithdrawMoneyOk, reqwest::Error>{
        let url = self.trading_sub_url.clone() + "withdrawals/";
        let mut params = HashMap::new();
        params.insert("amount", amount.to_string());

        let res: WithdrawMoneyOk = post_request(&url, &self.api_key, params).await?;

        return Ok(res);
    }

    /// limit: pagination limit 
    /// 
    /// page: results page 
    pub async fn retrieve_withdrawals(&self, limit: Option<i32>, page: Option<i32>) -> Result<Withdrawals, reqwest::Error>{
        let url = self.trading_sub_url.clone() + "withdrawals/";
        
        let mut query_params: Vec<(&str, String)> = vec![];
        if limit.is_some(){
            query_params.push(("limit", limit.unwrap().to_string()));
        }
        if page.is_some(){
            query_params.push(("page", page.unwrap().to_string()));
        }
        
        let withdrawals: Withdrawals = get_request(&url, &self.api_key,query_params).await?;
        Ok(withdrawals)
    }

    pub async fn retrieve_documents(&self) -> Result<Documents, reqwest::Error>{
        let url = self.trading_sub_url.clone() + "documents/";
        let docs: Documents = get_request(&url, &self.api_key, vec![]).await?;
        Ok(docs)
    }

    pub async fn download_document(&self, document_id: &str){
        let url = self.trading_sub_url.clone() + "documents/"+ document_id;
        //missing infos how the object looks

        //TODO Get
    }

    /// filter_type: filter for different types of bank statements: pay_in, pay_out, order_buy, order_sell, eod_balance, dividend
    /// 
    /// from: filter for bank statements after a specific date, format: "YYYY-MM-DD"
    /// 
    /// to: filter for bank statements until a specific date, format: "YYYY-MM-DD"
    /// 
    /// sorting: sort bank statements in ascending or descending order: asc, desc
    /// 
    /// limit: pagination limit 
    /// 
    /// page: results page 
    pub async fn retireve_bank_statements(&self, filter_type: Option<String>, from: Option<String>,
        to: Option<String>, sorting: Option<String>, limit: Option<i32>, page: Option<i32>) -> Result<BankStatements, reqwest::Error> {
        
        let mut query_params: Vec<(&str, String)> = vec![];
        if filter_type.is_some(){
            query_params.push(("type", filter_type.unwrap()));
        }
        if from.is_some(){
            query_params.push(("from", from.unwrap()));
        }
        if to.is_some(){
            query_params.push(("to", to.unwrap()));
        }
        if sorting.is_some(){
            query_params.push(("sorting", sorting.unwrap()));
        }
        if limit.is_some(){
            query_params.push(("limit", limit.unwrap().to_string()));
        }
        if page.is_some(){
            query_params.push(("page", page.unwrap().to_string()));
        }

        let url = self.trading_sub_url.clone() + "bankstatements/";
        let bank_statements: BankStatements = get_request(&url, &self.api_key, query_params).await?;
        Ok(bank_statements)
            //TODO Get
    }


}


async fn get_request<T: de::DeserializeOwned>(url: &String, api_key: &str, query_params: Vec<(&str, String)>) -> Result<T, reqwest::Error>{
    return Ok(reqwest::Client::new().get(url).query(&query_params)
        .header("Content-type", "application/x-www-form-urlencoded")
        .header("Authorization", "Bearer ".to_owned() + &api_key)
        .send()
        .await?
        .json::<T>()
        .await?);
}

async fn post_request<T: de::DeserializeOwned>(url: &String, api_key: &str, body_params: HashMap<&str, String>) -> Result<T, reqwest::Error>{
    return Ok(reqwest::Client::new().post(url)
    .header("Content-type", "application/x-www-form-urlencoded")
    .header("Authorization", "Bearer ".to_owned() + api_key)
    .form(&body_params)
    .send()
    .await?
    .json()
    .await?);
}


#[derive(Deserialize, Debug)]
pub struct AccountInformation{
    pub time: String,
    pub mode: String,
    pub status: String,
    pub results: AccountResults
}


#[derive( Deserialize, Debug)]
pub struct AccountResults{
    pub created_at: String,
    pub account_id: String,
    pub firstname: String,
    pub lastname: Option<String>,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub billing_address: Option<String>,
    pub billing_email: Option<String>,
    pub billing_name: Option<String>,
    pub billing_vat: Option<String>,
    pub mode: String,
    pub deposit_id: Option<String>,
    pub client_id: Option<String>,
    pub account_number: Option<String>,
    pub iban_brokerage: Option<String>,
    pub iban_origin: Option<String>,
    pub bank_name_origin: Option<String>,
    pub balance: i64, //(Your end-of-day balance from the day before) + (amount_sold_intraday) - (amount_bought_intraday) - (amount_open_withdrawals).
    pub cash_to_invest: i64, //(balance) - (amount_open_orders)
    pub cash_to_withdraw: i64, // (Your end-of-day balance from the day before) - (amount_bought_intraday) - (amount_open_withdrawals) - (amount_open_orders).
    pub amount_bought_intraday: i64,
    pub amount_sold_intraday: i64,
    pub amount_open_orders: i64,
    pub amount_open_withdrawals: i64,
    pub amount_estimate_taxes: i64,
    pub approved_at: Option<String>, //Time
    pub trading_plan: String,
    pub data_plan:String,
    pub tax_allowance: Option<i64>,
    pub tax_allowance_start: Option<String>,
    pub tax_allowance_end: Option<String>
}


#[derive(Deserialize, Debug)]
pub struct Withdrawals{
    pub time: String,
    pub status: String,
    pub mode: String,
    pub results: Vec<WithdrawalResult>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub total: i64,
    pub page: i32,
    pub pages: i32
}

#[derive(Deserialize, Debug)]
pub struct WithdrawalResult{
    pub id: String,
    pub amount: i64,
    pub created_at: String,
    pub date: Option<String>,
    pub idempotency: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Documents{
    pub time: String,
    pub status: String,
    pub mode: String,
    pub results: Vec<DocumentResult>,
}

#[derive(Deserialize, Debug)]
pub struct DocumentResult{
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub link: String,
    pub viewed_first_at: String,
    pub viewed_last_at: String
}


#[derive(Deserialize, Debug)]
pub struct BankStatements{
    pub time: String,
    pub status: String,
    pub mode: String,
    pub results: Vec<BankStatementResult>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub total: i64,
    pub page: i32,
    pub pages: i32
}

#[derive(Deserialize, Debug)]
pub struct BankStatementResult{
    pub id: String,
    pub account_id: String,
    #[serde(alias = "type")]
    pub type_name: String,
    pub date: String,
    pub amount: i64,
    pub isin: Option<String>,
    pub isin_title: Option<String>,
    pub created_at: String,
    pub quantity: Option<i64>
}

#[derive(Deserialize, Debug)]
pub struct WithdrawMoneyOk{
    pub time: String,
    pub status: String,
    pub mode: String
}


#[derive(Debug)]
pub struct Orders{
    trading_sub_url: String,
    api_key: String
}

impl Orders {
    
}


#[derive(Debug)]
pub struct Positions{
    trading_sub_url: String,
    api_key: String
}

impl Positions {
    
}



#[derive(Debug)]
pub struct Venues{
    trading_sub_url: String,
    api_key: String
}

impl Venues {
    pub async fn retireve_venues(&self) -> Result<VenueMarkets, reqwest::Error>{
        println!("{}",&self.trading_sub_url);
        let venue_markets: VenueMarkets = get_request(&self.trading_sub_url, &self.api_key, vec![]).await?;
        Ok(venue_markets)
    }
}

#[derive(Deserialize, Debug)]
pub struct VenueMarkets{
    pub time: String,
    pub results: Vec<VenueResult>,
    pub previous: Option<String>,
    pub next: Option<String>,
    pub total: i64,
    pub page: i32,
    pub pages: i32
}

#[derive(Deserialize, Debug)]
pub struct VenueResult{
    pub name: String,
    pub title: String,
    pub mic: String,
    pub is_open: bool,
    pub opening_hours: OpeningHours,
    pub opening_days: Vec<String>
}

#[derive(Deserialize, Debug)]
pub struct OpeningHours{
    pub start: String,
    pub end: String,
    pub timezone: String
}