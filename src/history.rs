use chrono::{DateTime, NaiveDateTime, Utc};
use crate::{Bar, Error, error, Interval, chart};
use snafu::{ensure};

fn aggregate_bars(data: chart::Result) -> Result<Vec<Bar>, Error> {
   let timestamps = &data.timestamps;
   let quotes = &data.indicators.quotes[0];

   ensure!(timestamps.len() == quotes.volume.len(), error::Other{ code: "Bad Data".to_string(), description: "Dates do not line up with quotes".to_string() });

   let mut result = Vec::new();
   for i in 0..timestamps.len() {
      // skip days where we have incomplete data
      if quotes.open[i].is_none() || quotes.high[i].is_none() || quotes.low[i].is_none() || quotes.close[i].is_none() || quotes.volume[i].is_none() { continue; }

      result.push(Bar {
         timestamp: DateTime::from_utc(NaiveDateTime::from_timestamp(timestamps[i], 0), Utc),
         open: quotes.open[i].unwrap(),
         high: quotes.high[i].unwrap(),
         low: quotes.low[i].unwrap(),
         close: quotes.close[i].unwrap(),
         volume: quotes.volume[i].unwrap()
      })
   }
   return Ok(result);
}

/// Retrieves (at most) 6 months worth of OCLHV data for a symbol
/// ending on the last market close.
/// 
/// # Examples
/// 
/// Get 6 months worth of Apple data:
/// 
/// ```
/// use yahoo_finance::history;
/// let data = history::retrieve("AAPL").unwrap();
/// for bar in &data {
///    println!("On {} Apple closed at ${:.2}", bar.timestamp.format("%b %e %Y"), bar.close)
/// }
/// ```
pub fn retrieve(symbol: &str) -> Result<Vec<Bar>, Error> {
   match chart::load_daily(symbol, Interval::_6mo) {
      Err(error) => return Err(error),
      Ok(data) => aggregate_bars(data)
   }
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
/// ```
/// use yahoo_finance::{history, Interval};
/// let data = history::retrieve_interval("AAPL", Interval::_5d).unwrap();
/// for bar in &data {
///    println!("On {} Apple closed at ${:.2}", bar.timestamp.format("%b %e %Y"), bar.close)
/// }
/// ```
pub fn retrieve_interval(symbol: &str, interval: Interval) -> Result<Vec<Bar>, Error> {
   // pre-conditions
   ensure!(!interval.is_intraday(), error::NoIntraday { interval });

   match chart::load_daily(symbol, interval) {
      Err(error) => return Err(error),
      Ok(data) => aggregate_bars(data)
   }
}

/// Retrieves OCLHV data for a symbol between a start and end date.
/// 
/// # Examples
/// 
/// Get 5 days worth of Apple data:
/// 
/// ```
/// use chrono::{Duration, TimeZone, Utc};
/// use yahoo_finance::{history};
/// 
/// let now = Utc::now();
/// let data = history::retrieve_range("AAPL", now - Duration::days(30), Some(now - Duration::days(10))).unwrap();
/// for bar in &data {
///    println!("On {} Apple closed at ${:.2}", bar.timestamp.format("%b %e %Y"), bar.close)
/// }
/// ```
pub fn retrieve_range(symbol: &str, start: DateTime<Utc>, end: Option<DateTime<Utc>>) -> Result<Vec<Bar>, Error> {
   // pre-conditions
   let _end = end.unwrap_or(Utc::now());
   println!("{} {}", start, _end);
   ensure!(_end.signed_duration_since(start).num_seconds() > 0, error::InvalidStartDate);

   match chart::load_daily_range(symbol, start.timestamp(), _end.timestamp()) {
      Err(error) => return Err(error),
      Ok(data) => aggregate_bars(data)
   }
}