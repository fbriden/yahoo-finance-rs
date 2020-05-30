use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use reqwest::Url;
use serde::Deserialize;
use snafu::{ ensure, OptionExt, ResultExt };
use std::env;

use crate::{error, Interval, Result};

const BASE_URL: &'static str = "https://query1.finance.yahoo.com/v8/finance/chart/";

/// Helper function to build up the main query URL
fn build_query(symbol: &str) -> Result<Url> {
   let base = env::var("TEST_URL").unwrap_or(BASE_URL.to_string());
   Ok(Url::parse(&base).context(error::InternalURL { url: &base })?
      .join(symbol).context(error::InternalURL { url: symbol })?)
}

ez_serde!(Meta {
   symbol: String,

   #[serde(with = "ts_seconds")]
   first_trade_date: DateTime<Utc>,

   #[serde(rename = "regularMarketPrice")]
   current_price: f32,

   #[serde(rename = "chartPreviousClose")]
   previous_close: f32
});
ez_serde!(OHLCV { open: Vec<Option<f64>>, high: Vec<Option<f64>>, low: Vec<Option<f64>>, close: Vec<Option<f64>>, volume: Vec<Option<u64>> });
ez_serde!(Indicators { #[serde(rename = "quote")] quotes: Vec<OHLCV> });
ez_serde!(Data { meta: Meta, #[serde(rename = "timestamp")] timestamps: Vec<i64>, indicators: Indicators });

ez_serde!(Error {code: String, description: String });
ez_serde!(Chart { result: Option<Vec<Data>>, error: Option<Error> });
ez_serde!(Response { chart: Chart });

async fn load(url: &Url) -> Result<Data> {
   // make the call - we do not really expect this to fail.
   // ie - we won't 404 if the symbol doesn't exist
   let response = reqwest::get(url.clone()).await.context(error::RequestFailed)?;
   ensure!(
      response.status().is_success(),
      error::CallFailed{ url: response.url().to_string(), status: response.status().as_u16() }
   );

   let data = response.text().await.context(error::UnexpectedErrorRead { url: url.to_string() })?;
   let chart = serde_json::from_str::<Response>(&data).context(error::BadData)?.chart;

   if !chart.result.is_some() {
      // no result so we'd better have an error
      let err = chart.error.context(error::InternalLogic{ reason: "error block exists without values"})?;
      error::ChartFailed{ code: err.code, description: err.description }.fail()?;
   }

   // we have a result to process
   let result = chart.result.context(error::UnexpectedErrorYahoo)?;
   ensure!(result.len() > 0, error::UnexpectedErrorYahoo);
   Ok(result[0].clone())
}

pub async fn load_daily(symbol: &str, period: Interval) -> Result<Data> {
   let mut lookup = build_query(symbol)?;
   lookup.query_pairs_mut()
      .append_pair("range", &period.to_string())
      .append_pair("interval", "1d");

   load(&lookup).await
}

pub async fn load_daily_range(symbol: &str, start: i64, end: i64) -> Result<Data> {
   let mut lookup = build_query(symbol)?;
   lookup.query_pairs_mut()
      .append_pair("period1", &start.to_string())
      .append_pair("period2", &end.to_string())
      .append_pair("interval", "1d");

   load(&lookup).await
}
