//! # Yahoo Finance
//! 
//! Yahoo! provides some great market data and this is a library to easily get
//! that information out of Yahoo for use in financial applications.
//! 
//! Currently `yahoo_finance` provides:
//! * Historical quote information [OHCL Data](https://en.wikipedia.org/wiki/Open-high-low-close_chart) + volume
//! * Relatively real-time quote informaton with comparible performance to the real-time updates on their website
//! 
//! ## Quick Examples
//! 
//! To retrieve the intraday high for the last 3 months of Apple you can use something like:
//! 
//! ```no_run
//! use yahoo_finance::{history, Interval};
//! 
//! // retrieve 6 months worth of data
//! let data = history::retrieve_interval("AAPL", Interval::_6mo).unwrap();
//! 
//! // print out some high numbers!
//! for bar in &data {
//!    println!("Apple hit an intraday high of ${:.2} on {}.",
//!      bar.high, bar.timestamp.format("%b %e %Y")
//!    )
//! }
//! ```
//!
//! To listen on relatively real-time changes in price:
//! 
//! ```no_run
//! use yahoo_finance::realtime::{ Quote, Streamer };
//!
//! fn print_quote(quote: Quote) {
//!    println!("At {}, {} is trading for ${}", quote.timestamp, quote.symbol, quote.price)
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!    let mut streamer = Streamer::new().await;
//!
//!    streamer.subscribe(vec!["AAPL", "IBM", "^DJI", "^IXIC"], print_quote).await;
//!    streamer.run().await?;
//!
//!    Ok(())
//! }
//! ```

// make sure our macros are all loaded
#[macro_use] mod macros;

use chrono::{DateTime, Utc};

/// A single 'bar' of price information containing OHLCV data
#[derive(Debug)]
pub struct Bar {
   pub timestamp: DateTime<Utc>,
   pub open: f64,
   pub high: f64,
   pub low: f64,
   pub close: f64,
   pub volume: u64
}

mod interval;
mod error;
mod chart;

/// Historical quotes
pub mod history;

/// Realtime quotes
pub mod realtime;

// re-export stuff for external use
pub use interval::{Interval};
pub use error::{Error};
