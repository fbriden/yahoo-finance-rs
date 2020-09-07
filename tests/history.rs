use chrono::{Duration, Utc};
use mockito::{mock, Mock};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio_test::block_on;
use yahoo_finance::{history, Interval};

fn base_mock(test_name: &str, symbol: &str, query: &str) -> std::io::Result<Mock> {
   // Tell the actual code to use a test URL rather than the live one
   env::set_var("TEST_URL", mockito::server_url());

   // Load the simulated Yahoo data we want to test against
   let mut file = File::open(format!("tests/history_data/{}.json", test_name))?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;

   // Serve up the test data on the test URL
   Ok(mock("GET", format!("/{symbol}?{query}", symbol=symbol, query=query).as_str())
      .with_header("content-type", "application/json")
      .with_body(&contents)
      .with_status(200))
}

fn build_interval(interval: Interval) -> String { format!("range={r}&interval={i}", r=interval, i=Interval::_1d) }

#[test]
fn retrieve_valid() {
   //! Ensure that we can load for valid companies

   // GIVEN - a valid response and stock symbol
   let symbol = "AAPL";
   let _m = base_mock("aapl", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we load the data
   let result = block_on(history::retrieve(symbol)).unwrap();
   assert!(result.len() > 0)
}

#[test]
#[should_panic(expected = "code: \"Not Found\"")]
fn retrieve_invalid_symbol() {
   //! Ensure that we gracefully fail when retrieving data for an invalid symbol

   // GIVEN - a valid response for an invalid symbol
   let symbol = "FUBAR";
   let _m = base_mock("not_found", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we load the data
   block_on(history::retrieve(symbol)).unwrap();

   // THEN - we get an error
}

#[test]
#[should_panic(expected = "NoIntraday")]
fn retrieve_interval_invalid() {
   //! Ensure that we gracefully fail when we use an intraday interval

   // GIVEN - a valid response for an valid symbol
   let symbol = "AAPL";
   let _m = base_mock("aapl", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we get a date range where the start date is after the end date
   block_on(history::retrieve_interval(symbol, Interval::_1m)).unwrap();

   // THEN - we get an error
}

#[test]
#[should_panic(expected = "InvalidStartDate")]
fn retrieve_range_invalid1() {
   //! Ensure that we gracefully fail when we use no end date before the start date

   // GIVEN - a valid response for an valid symbol
   let symbol = "AAPL";
   let _m = base_mock("aapl", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we get a date range where the start date is after the end date
   block_on(history::retrieve_range(symbol, Utc::now() - Duration::days(10), Some(Utc::now() - Duration::days(15)))).unwrap();

   // THEN - we get an error
}

#[test]
#[should_panic(expected = "InvalidStartDate")]
fn retrieve_range_invalid2() {
   //! Ensure that we gracefully fail when we use no end date and a start date in the future

   // GIVEN - a valid response for an valid symbol
   let symbol = "AAPL";
   let _m = base_mock("aapl", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we get a date range where the start date is after the end date
   block_on(history::retrieve_range(symbol, Utc::now() + Duration::days(10), None)).unwrap();

   // THEN - we get an error
}

#[test]
fn retrieve_no_quote_data() {
   //! Ensure that we gracefully handle the case where Yahoo send us an empty dictionary
   //! of quote data

   // GIVEN - a valid response with no timestmap & no quote data
   let symbol = "AAPL";
   let _m = base_mock("no_quote_data", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we get data where the there is basically no data
   let result = block_on(history::retrieve(symbol)).unwrap();
   assert!(result.len() == 0)
}

#[test]
#[should_panic(expected = "no timestamps")]
fn retrieve_no_timestamp_data() {
   //! Ensure that we gracefully handle the case where Yahoo send us an empty dictionary
   //! of quote data

   // GIVEN - a valid response with an empty dictionary of quote data and no timestamps
   let symbol = "AAPL";
   let _m = base_mock("no_timestamp_data", symbol, build_interval(Interval::_6mo).as_str()).unwrap().create();

   // WHEN - we get data where the there are no quotes
   block_on(history::retrieve(symbol)).unwrap();

   // THEN - we get an error
}
