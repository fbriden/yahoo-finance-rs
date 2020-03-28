# Yahoo Finance for Rust

A Rust library for getting financial information from [Yahoo!](https://finance.yahoo.com/)

[![Package][cratesio-image]][cratesio]
[![Documentation][docsrs-image]][docsrs]
[![Build Status][build-image]][build]

[docsrs-image]: https://docs.rs/yahoo-finance-rs/badge.svg
[docsrs]: https://docs.rs/yahoo-finance-rs
[cratesio-image]: https://img.shields.io/crates/v/yahoo-finance-rs.svg
[cratesio]: https://crates.io/crates/yahoo-finance-rs
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

### Usage

Add this to your `Cargo.toml`:

```toml
yahoo-finance-rs = "0.1"
```
