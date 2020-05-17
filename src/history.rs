use chrono::{DateTime, Utc};
use snafu::{ensure, OptionExt};

use crate::{error, yahoo, Bar, Interval, Result};

fn aggregate_bars(data: yahoo::Data) -> Result<Vec<Bar>> {
   let timestamps = &data.timestamps;
   let quotes = &data.indicators.quotes;
   ensure!(!quotes.is_empty(), error::MissingData { reason: "missing quotes data" });

   let quote = &quotes[0];
   ensure!(timestamps.len() == quote.volume.len(), error::MissingData { reason: "dates do not line up with quotes" });

   let mut result = Vec::new();
   #[allow(clippy::needless_range_loop)]
   for i in 0..timestamps.len() {
      // skip days where we have incomplete data
      if quote.open[i].is_none() || quote.high[i].is_none() || quote.low[i].is_none() || quote.close[i].is_none() {
         continue;
      }

      result.push(Bar {
         timestamp: timestamps[i] * 1000,
         open: quote.open[i].context(error::InternalLogic{ reason: "missing open not caught" })?,
         high: quote.high[i].context(error::InternalLogic{ reason: "missing high not caught" })?,
         low: quote.low[i].context(error::InternalLogic{ reason: "missing low not caught" })?,
         close: quote.close[i].context(error::InternalLogic{ reason: "missing close not caught" })?,
         volume: quote.volume[i],
      })
   }
   Ok(result)
}

/// Retrieves (at most) 6 months worth of OCLHV data for a symbol
/// ending on the last market close.
///
/// # Examples
///
/// Get 6 months worth of Apple data:
///
/// ``` no-run
/// use yahoo_finance::{ history, Timestamped };
///
/// let data = history::retrieve("AAPL").unwrap();
/// for bar in &data {
///    println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
/// }
/// ```
pub async fn retrieve(symbol: &str) -> Result<Vec<Bar>> {
   aggregate_bars(yahoo::load_daily(symbol, Interval::_6mo).await?)
}

/// Retrieves a configurable amount of OCLHV data for a symbol
/// ending on the last market close.  The amount of data returned
/// might be less than the interval specified if the symbol
/// is new.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no-run
/// use yahoo_finance::{ history, Interval, Timestamped };
///
/// let data = history::retrieve_interval("AAPL", Interval::_5d).unwrap();
/// for bar in &data {
///    println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
/// }
/// ```
pub async fn retrieve_interval(symbol: &str, interval: Interval) -> Result<Vec<Bar>> {
   // pre-conditions
   ensure!(!interval.is_intraday(), error::NoIntraday { interval });

   aggregate_bars(yahoo::load_daily(symbol, interval).await?)
}

/// Retrieves OCLHV data for a symbol between a start and end date.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no-run
/// use chrono::{Duration, TimeZone, Utc};
/// use yahoo_finance::{ history, Timestamped };
///
/// let now = Utc::now();
/// let data = history::retrieve_range("AAPL", now - Duration::days(30), Some(now - Duration::days(10))).unwrap();
/// for bar in &data {
///    println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
/// }
/// ```
pub async fn retrieve_range(symbol: &str, start: DateTime<Utc>, end: Option<DateTime<Utc>>) -> Result<Vec<Bar>> {
   // pre-conditions
   let _end = end.unwrap_or_else(Utc::now);
   ensure!(_end.signed_duration_since(start).num_seconds() > 0, error::InvalidStartDate);

   aggregate_bars(yahoo::load_daily_range(symbol, start.timestamp(), _end.timestamp()).await?)
}
