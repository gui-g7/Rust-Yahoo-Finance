use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct ChartResponse {
    pub chart: Chart,
}

#[derive(Debug, Deserialize)]
pub struct Chart {
    pub result: Vec<ResultData>,
}

#[derive(Debug, Deserialize)]
pub struct ResultData {
    pub meta: MetaData,
    pub timestamp: Option<Vec<u64>>,
    pub indicators: Indicators,
    pub events: Option<Events>,
}

#[derive(Debug, Deserialize)]
pub struct MetaData {
    pub currency: String,
    pub symbol: String,
    pub exchange_name: String,
}

#[derive(Debug, Deserialize)]
pub struct Indicators {
    pub quote: Vec<QuoteData>,
}

#[derive(Debug, Deserialize)]
pub struct QuoteData {
    pub open: Option<Vec<f64>>,
    pub close: Option<Vec<f64>>,
    pub high: Option<Vec<f64>>,
    pub low: Option<Vec<f64>>,
    pub volume: Option<Vec<u64>>,
}

#[derive(Debug, Deserialize)]
pub struct Events {
    pub dividends: Option<HashMap<String, DividendData>>,
}

#[derive(Debug, Deserialize)]
pub struct DividendData {
    pub amount: f64,
    pub date: u64,
}
