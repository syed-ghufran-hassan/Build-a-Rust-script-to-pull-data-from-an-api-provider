use std::error::Error;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::env;

use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct TickerInfo {
    name: String,
    market: String,
    locale: String,
}

#[derive(Debug, Deserialize)]
struct Financials {
    #[serde(rename = "results")]
    data: Vec<FinancialData>,
}

#[derive(Debug, Deserialize, Serialize)]
struct FinancialData {
    ticker: String,
    period: String,
    calendar_date: String,
    #[serde(rename = "value")]
    amount: f64,
}

#[derive(Debug, Serialize)]
struct FinancialsCSV {
    ticker: String,
    period: String,
    calendar_date: String,
    amount: f64,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load API key from environment variables
    let api_key = env::var("POLYGON_API_KEY")?;
    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(&format!("Bearer {}", api_key))?,
    );

    // Read tickers from file
    let tickers_file = File::open("ticker.txt")?;
    let tickers_reader = std::io::BufReader::new(tickers_file);
    let tickers = tickers_reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| line.split('\t').next().map(|s| s.to_string()));

    // Iterate over tickers
    for ticker in tickers {
        let ticker_info_url = format!(
            "https://api.polygon.io/v3/reference/tickers/{}",
            ticker
        );
        let ticker_info: TickerInfo = get_json(&ticker_info_url, &headers).await?;

        let financials_url = format!(
            "https://api.polygon.io/vX/reference/financials?ticker={}",
            ticker
        );
        let financials: Financials = get_json(&financials_url, &headers).await?;

        // Extract data from JSON
        let financials_csv: Vec<FinancialsCSV> = financials
            .data
            .iter()
            .map(|data| FinancialsCSV {
                ticker: ticker.clone(),
                period: data.period.clone(),
                calendar_date: data.calendar_date.clone(),
                amount: data.amount,
            })
            .collect();

        // Write data to CSV file
        let mut csv_file = BufWriter
