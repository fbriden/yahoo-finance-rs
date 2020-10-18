use chrono::{DateTime, Utc};
use snafu::{ensure, OptionExt};

use crate::{error, yahoo, Bar, Interval, Result};

fn extract_events(data: yahoo::Data) -> (Vec<yahoo::Dividend>, Vec<yahoo::Split>) {
    let events = match data.events {
        None => return (Vec::new(), Vec::new()),
        Some(events) => events,
    };
    // The API returns events as a map by date; here we simply flatten to a `Vec`
    let dividends = events.dividends.map(|ds| ds.into_iter().map(|(_date, mut dividend)| { dividend.timestamp *= 1000; dividend }).collect()).unwrap_or_else(Vec::new);
    let splits = events.splits.map(|ss| ss.into_iter().map(|(_date, mut split)| { split.timestamp *= 1000; split }).collect()).unwrap_or_else(Vec::new);
    (dividends, splits)
}

fn aggregate_bars_and_extract_events(data: yahoo::Data) -> Result<(Vec<Bar>, Vec<yahoo::Dividend>, Vec<yahoo::Split>)> {
   let mut result = Vec::new();
   let timestamps = &data.timestamps;
   let quotes = &data.indicators.quotes;

   // if we have no timestamps & no quotes we'll assume there is no data
   if timestamps.is_empty() && quotes.is_empty() {
       let (dividends, splits) = extract_events(data);
       return Ok((result, dividends, splits));
   }

   // otherwise see if one is empty and reflects bad data from Yahoo!
   ensure!(!timestamps.is_empty(), error::MissingData { reason: "no timestamps for OHLCV data" });
   ensure!(!quotes.is_empty(), error::MissingData { reason: "no OHLCV data" });

   // make sure timestamps lines up with the OHLCV data
   let quote = &quotes[0];
   ensure!(timestamps.len() == quote.volumes.len(), error::MissingData { reason: "timestamps do not line up with OHLCV data" });
   ensure!(timestamps.len() == quote.opens.len(), error::MissingData { reason: "'open' values do not line up the timestamps" });
   ensure!(timestamps.len() == quote.highs.len(), error::MissingData { reason: "'high' values do not line up the timestamps" });
   ensure!(timestamps.len() == quote.lows.len(), error::MissingData { reason: "'low' values do not line up the timestamps" });
   ensure!(timestamps.len() == quote.closes.len(), error::MissingData { reason: "'close' values do not line up the timestamps" });

   #[allow(clippy::needless_range_loop)]
   for i in 0..timestamps.len() {
      // skip days where we have incomplete data
      if quote.opens[i].is_none() || quote.highs[i].is_none() || quote.lows[i].is_none() || quote.closes[i].is_none() {
         continue;
      }

      result.push(Bar {
         timestamp: timestamps[i] * 1000,
         open: quote.opens[i].context(error::InternalLogic{ reason: "missing open not caught" })?,
         high: quote.highs[i].context(error::InternalLogic{ reason: "missing high not caught" })?,
         low: quote.lows[i].context(error::InternalLogic{ reason: "missing low not caught" })?,
         close: quote.closes[i].context(error::InternalLogic{ reason: "missing close not caught" })?,
         volume: quote.volumes[i],
      })
   }

   let (dividends, splits) = extract_events(data);
   Ok((result, dividends, splits))
}

/// Retrieves (at most) 6 months worth of OCLHV data for a symbol
/// ending on the last market close.
///
/// # Examples
///
/// Get 6 months worth of Apple data:
///
/// ``` no_run
/// use yahoo_finance::{ history, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    match history::retrieve("AAPL").await {
///       Err(e) => println!("Failed to call Yahoo: {:?}", e),
///       Ok(data) => 
///          for bar in &data {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///    }
/// }
/// ```
pub async fn retrieve(symbol: &str) -> Result<Vec<Bar>> {
   aggregate_bars_and_extract_events(yahoo::load_daily(symbol, Interval::_6mo).await?).map(|(bars, _dividends, _splits)| bars)
}

/// Retrieves (at most) 6 months worth of OCLHV and corporate action event data for a symbol
///
/// # Examples
///
/// Get any Apple corporate actions within the last 6 months:
///
/// ``` no_run
/// use yahoo_finance::{ history, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    match history::retrieve_with_events("AAPL").await {
///       Err(e) => println!("Failed to call Yahoo: {:?}", e),
///       Ok((bars, dividends, splits)) => {
///          for bar in &bars {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///          for dividend in &dividends {
///             println!("Apple dividend of {} on {}", dividend.amount, dividend.datetime().format("%b %e %Y"))
///          }
///          for split in &splits {
///             println!("Apple split {}:{} on {}", split.numerator, split.denominator, split.datetime().format("%b %e %Y"))
///          }
///       }
///    }
/// }
/// ```
pub async fn retrieve_with_events(symbol: &str) -> Result<(Vec<Bar>, Vec<yahoo::Dividend>, Vec<yahoo::Split>)> {
    yahoo::load_daily_with_events(symbol, Interval::_6mo).await.and_then(aggregate_bars_and_extract_events)
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
/// ``` no_run
/// use yahoo_finance::{ history, Interval, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    match history::retrieve_interval("AAPL", Interval::_5d).await {
///       Err(e) => println!("Failed to call Yahoo: {:?}", e),
///       Ok(data) => 
///          for bar in &data {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///    }
/// }
/// ```
pub async fn retrieve_interval(symbol: &str, interval: Interval) -> Result<Vec<Bar>> {
   // pre-conditions
   ensure!(!interval.is_intraday(), error::NoIntraday { interval });

   aggregate_bars_and_extract_events(yahoo::load_daily(symbol, interval).await?).map(|(bars, _dividends, _splits)| bars)
}

/// Retrieves a configurable amount of OCLHV and corporate action event data for a symbol.  The amount of
/// data returned might be less than the interval specified if the symbol is new.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no_run
/// use yahoo_finance::{ history, Interval, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    match history::retrieve_interval_with_events("AAPL", Interval::_5d).await {
///       Err(e) => println!("Failed to call Yahoo: {:?}", e),
///       Ok((bars, dividends, splits)) => {
///          for bar in &bars {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///          for dividend in &dividends {
///             println!("Apple dividend of {} on {}", dividend.amount, dividend.datetime().format("%b %e %Y"))
///          }
///          for split in &splits {
///             println!("Apple split {}:{} on {}", split.numerator, split.denominator, split.datetime().format("%b %e %Y"))
///          }
///       }
///    }
/// }
/// ```
pub async fn retrieve_interval_with_events(symbol: &str, interval: Interval) -> Result<(Vec<Bar>, Vec<yahoo::Dividend>, Vec<yahoo::Split>)> {
   // pre-conditions
   ensure!(!interval.is_intraday(), error::NoIntraday { interval });

   yahoo::load_daily_with_events(symbol, interval).await.and_then(aggregate_bars_and_extract_events)
}

/// Retrieves OCLHV data for a symbol between a start and end date.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no_run
/// use chrono::{Duration, TimeZone, Utc};
/// use yahoo_finance::{ history, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    let now = Utc::now();
///    match history::retrieve_range("AAPL", now - Duration::days(30), Some(now - Duration::days(10))).await {
///       Err(e) => println!("Failed to call Yahoo {:?}", e),
///       Ok(data) =>
///          for bar in &data {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///    }
/// }
/// ```
pub async fn retrieve_range(symbol: &str, start: DateTime<Utc>, end: Option<DateTime<Utc>>) -> Result<Vec<Bar>> {
   // pre-conditions
   let _end = end.unwrap_or_else(Utc::now);
   ensure!(_end.signed_duration_since(start).num_seconds() > 0, error::InvalidStartDate);

   aggregate_bars_and_extract_events(yahoo::load_daily_range(symbol, start.timestamp(), _end.timestamp()).await?).map(|(bars, _dividends, _splits)| bars)
}

/// Retrieves OCLHV and corporate action event data for a symbol between a start and end date.
///
/// # Examples
///
/// Get 5 days worth of Apple data:
///
/// ``` no_run
/// use chrono::{Duration, TimeZone, Utc};
/// use yahoo_finance::{ history, Timestamped };
///
/// #[tokio::main]
/// async fn main() {
///    let now = Utc::now();
///    match history::retrieve_range_with_events("AAPL", now - Duration::days(30), Some(now - Duration::days(10))).await {
///       Err(e) => println!("Failed to call Yahoo {:?}", e),
///       Ok((bars, dividends, splits)) => {
///          for bar in &bars {
///             println!("On {} Apple closed at ${:.2}", bar.datetime().format("%b %e %Y"), bar.close)
///          }
///          for dividend in &dividends {
///             println!("Apple dividend of {} on {}", dividend.amount, dividend.datetime().format("%b %e %Y"))
///          }
///          for split in &splits {
///             println!("Apple split {}:{} on {}", split.numerator, split.denominator, split.datetime().format("%b %e %Y"))
///          }
///       }
///    }
/// }
/// ```
pub async fn retrieve_range_with_events(symbol: &str, start: DateTime<Utc>, end: Option<DateTime<Utc>>) -> Result<(Vec<Bar>, Vec<yahoo::Dividend>, Vec<yahoo::Split>)> {
   // pre-conditions
   let _end = end.unwrap_or_else(Utc::now);
   ensure!(_end.signed_duration_since(start).num_seconds() > 0, error::InvalidStartDate);

   yahoo::load_daily_range_with_events(symbol, start.timestamp(), _end.timestamp()).await.and_then(aggregate_bars_and_extract_events)
}
