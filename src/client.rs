use crate::models::ChartResponse;
use reqwest::blocking::get;
use std::error::Error;

const BASE_URL: &str = "https://query1.finance.yahoo.com/v8/finance/chart";

pub struct YahooFinanceClient;

impl YahooFinanceClient {
    pub fn get_stock_data(symbol: &str, interval: &str) -> Result<ChartResponse, Box<dyn Error>> {
        let url = format!("{}/{}?interval={}", BASE_URL, symbol, interval);
        let response = get(&url)?.json::<ChartResponse>()?;
        Ok(response)
    }

    pub fn get_dividends(symbol: &str, interval: &str) -> Result<Vec<(u64, f64)>, Box<dyn Error>> {
        let stock_data = Self::get_stock_data(symbol, interval)?;

        if let Some(events) = stock_data.chart.result[0].events.as_ref() {
            if let Some(dividends) = &events.dividends {
                let mut result = Vec::new();
                for (timestamp, data) in dividends {
                    result.push((timestamp.parse::<u64>()?, data.amount));
                }
                return Ok(result);
            }
        }
        Ok(vec![])
    }
}
