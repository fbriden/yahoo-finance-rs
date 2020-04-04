# Yahoo Finance for Rust

A Rust library for getting financial information from [Yahoo!](https://finance.yahoo.com/)

[![Package][cratesio-image]][cratesio]
[![Documentation][docsrs-image]][docsrs]
[![Build Status][build-image]][build]

[docsrs-image]: https://docs.rs/yahoo-finance/badge.svg
[docsrs]: https://docs.rs/yahoo-finance
[cratesio-image]: https://img.shields.io/crates/v/yahoo-finance.svg
[cratesio]: https://crates.io/crates/yahoo-finance
[build-image]: https://github.com/fbriden/yahoo-finance-rs/workflows/Build/badge.svg
[build]: https://github.com/fbriden/yahoo-finance-rs/actions

* Historical OHLCV pricing information

```rust
use yahoo_finance::history;

fn main() {
   // retrieve 6 months worth of data for Apple
   let data = history::retrieve("AAPL").unwrap();

   // print the date and closing price for each day we have data
   for bar in &data {
      println!("On {} Apple closed at ${:.2}", bar.timestamp.format("%b %e %Y"), bar.close)
   }
}
```

* Realtime pricing information

```rust
use yahoo_finance::{ Quote, Streamer };

fn print_quote(quote: Quote) {
   println!("At {}, {} is trading for ${}", quote.timestamp, quote.symbol, quote.price)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut streamer = Streamer::new().await;

   streamer.subscribe(vec!["AAPL", "^DJI", "^IXIC"], print_quote).await;
   streamer.run().await;

   Ok(())
}
```

### Usage

Add this to your `Cargo.toml`:

```toml
yahoo-finance = "0.2"
```
