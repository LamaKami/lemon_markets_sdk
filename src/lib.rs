use std::collections::HashMap;

use reqwest::{Error, Response};
use serde::{Deserialize, de};
//use serde_json::Result;

#[derive(Debug)]
pub struct LemonMarkets{
    pub account: Account,
    api_key: String,
    trading_url: String,
}

impl LemonMarkets {
    pub fn new(api_key: String, trading_url: String) -> LemonMarkets{
        LemonMarkets{account: Account{trading_sub_url: trading_url.clone()+"account/", api_key: api_key.clone()}
        , api_key, trading_url}
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
    time: String,
    mode: String,
    status: String,
    results: AccountResults
}


#[derive( Deserialize, Debug)]
pub struct AccountResults{
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
    balance: i64, //(Your end-of-day balance from the day before) + (amount_sold_intraday) - (amount_bought_intraday) - (amount_open_withdrawals).
    cash_to_invest: i64, //(balance) - (amount_open_orders)
    cash_to_withdraw: i64, // (Your end-of-day balance from the day before) - (amount_bought_intraday) - (amount_open_withdrawals) - (amount_open_orders).
    amount_bought_intraday: i64,
    amount_sold_intraday: i64,
    amount_open_orders: i64,
    amount_open_withdrawals: i64,
    amount_estimate_taxes: i64,
    approved_at: Option<String>, //Time
    trading_plan: String,
    data_plan:String,
    tax_allowance: Option<i64>,
    tax_allowance_start: Option<String>,
    tax_allowance_end: Option<String>
}


#[derive(Deserialize, Debug)]
pub struct Withdrawals{
    time: String,
    status: String,
    mode: String,
    results: Vec<WithdrawalResult>,
    previous: Option<String>,
    next: Option<String>,
    total: i64,
    page: i32,
    pages: i32
}

#[derive(Deserialize, Debug)]
pub struct WithdrawalResult{
    id: String,
    amount: i64,
    created_at: String,
    date: Option<String>,
    idempotency: Option<String>,
}



#[derive(Deserialize, Debug)]
pub struct Documents{
    time: String,
    status: String,
    mode: String,
    results: Vec<DocumentResult>,
}

#[derive(Deserialize, Debug)]
pub struct DocumentResult{
    id: String,
    name: String,
    created_at: String,
    link: String,
    viewed_first_at: String,
    viewed_last_at: String
}


#[derive(Deserialize, Debug)]
pub struct BankStatements{
    time: String,
    status: String,
    mode: String,
    results: Vec<BankStatementResult>,
    previous: Option<String>,
    next: Option<String>,
    total: i64,
    page: i32,
    pages: i32
}

#[derive(Deserialize, Debug)]
pub struct BankStatementResult{
    id: String,
    account_id: String,
    #[serde(alias = "type")]
    type_name: String,
    date: String,
    amount: i64,
    isin: Option<String>,
    isin_title: Option<String>,
    created_at: String,
    quantity: Option<i64>
}

#[derive(Deserialize, Debug)]
pub struct WithdrawMoneyOk{
    time: String,
    status: String,
    mode: String
}