//! # Yahoo Finance
//!
//! Yahoo! provides some great market data and this is a library to easily get
//! that information out of Yahoo for use in financial applications.
//!
//! Currently `yahoo_finance` provides:
//! * Historical quote information [OHCL Data](https://en.wikipedia.org/wiki/Open-high-low-close_chart) + volume
//! * Relatively real-time quote informaton with comparible performance to the real-time updates on their website
//! * Company profile information including address, sector, industry, etc.
//! 
//! ## Quick Examples
//!
//! To retrieve the intraday high for the last 3 months of Apple you can use something like:
//!
//! ```no_run
//! use yahoo_finance::{history, Interval, Timestamped};
//!
//! // retrieve 6 months worth of data
//! #[tokio::main]
//! async fn main() {
//!    let data = history::retrieve_interval("AAPL", Interval::_6mo).await.unwrap();
//!
//!    // print out some high numbers!
//!    for bar in &data {
//!       println!("Apple hit an intraday high of ${:.2} on {}.", bar.high, bar.datetime().format("%b %e %Y"));
//!    }
//! }
//! ```
//!
//! To listen on relatively real-time changes in price:
//!
//! ```no_run
//! use futures::{ future, StreamExt };
//! use yahoo_finance::Streamer;
//!
//! #[tokio::main]
//! async fn main() {
//!    let streamer = Streamer::new(vec!["AAPL", "QQQ", "^DJI", "^IXIC"]);
//!
//!    streamer.stream().await
//!       .for_each(|quote| {
//!          println!("At {}, {} is trading for ${}", quote.timestamp, quote.symbol, quote.price);
//!          future::ready(())
//!       })
//!       .await;
//! }
//! ```
//!
//! To get the industry & sectory for a symbol.
//!
//! ```no_run
//! use yahoo_finance::{Profile};
//!
//! #[tokio::main]
//! async fn main() {
//!    match Profile::load("AAPL").await.unwrap() {
//!       Profile::Company(profile) => println!("{}:{}", profile.industry.unwrap(), profile.sector.unwrap()),
//!       _ => {}
//!    }
//! }
//! ```

// make sure our macros are all loaded
#[macro_use]
mod macros;

pub use market_finance::{Bar, Interval, Quote, Timestamped, TradingSession};

mod error;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub struct Error(error::InnerError);

pub type Result<T> = std::result::Result<T, Error>;

mod yahoo;

/// Historical quotes
pub mod history;

/// Realtime quotes
mod streaming;
pub use streaming::Streamer;

/// Symbol profile
mod profile;
pub use profile::Profile;