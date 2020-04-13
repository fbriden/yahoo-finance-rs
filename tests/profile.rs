use mockito::{mock, Mock};
use std::env;
use std::fs::File;
use std::io::prelude::*;
use tokio_test::block_on;
use yahoo_finance::{Profile};

fn base_mock(test_name: &str, symbol: &str) -> std::io::Result<Mock> {
   // Tell the actual code to use a test URL rather than the live one
   env::set_var("TEST_URL", mockito::server_url());

   // Load the simulated Yahoo data we want to test against
   let mut file = File::open(format!("tests/profile_data/{}.html", test_name))?;
   let mut contents = String::new();
   file.read_to_string(&mut contents)?;

   // Serve up the test data on the test URL
   Ok(mock("GET", format!("/quote/{symbol}?p={symbol}", symbol=symbol).as_str())
      .with_header("content-type", "text/html")
      .with_body(&contents)
      .with_status(200))
}

#[test]
fn load_company() {
   //! Ensure that we can load for valid companies

   // GIVEN - a valid response and stock symbol
   let symbol = "AAPL";
   let _m = base_mock("aapl", symbol).unwrap().create();

   // WHEN - we load the data
   let result = block_on(Profile::load(symbol)).unwrap();

   // THEN - we get the results we expect
   match result {
      Profile::Company(profile) => {
         assert_eq!("Apple Inc.", profile.name);
         // assert_eq!("Consumer Electronics", result.industry);
         // assert_eq!("Technology", result.sector);
         // assert_eq!("http://www.apple.com", result.website);
      },
      _ => panic!("Needs to be a company profile")
   }
}

#[test]
fn load_fund() {
   //! Ensure that we can load for valid funds

   // GIVEN - a valid response and stock symbol
   let symbol = "QQQ";
   let _m = base_mock("qqq", symbol).unwrap().create();

   // WHEN - we load the data
   let result = block_on(Profile::load(symbol)).unwrap();

   // THEN - we get the results we expect
   match result {
      Profile::Fund(profile) => {
         assert_eq!("Invesco QQQ Trust", profile.name);
         // assert_eq!("Consumer Electronics", result.industry);
         // assert_eq!("Technology", result.sector);
         // assert_eq!("http://www.apple.com", result.website);
      },
      _ => panic!("Needs to be a fund  profile")
   }
}

#[test]
#[should_panic(expected = "BadData")]
fn load_bad_data() {
   //! Ensures that we gracefully fail when Yahoo! sends back bad JSON

   // GIVEN - a response that has no data
   let symbol = "NULL";
   let _m = base_mock("invalid_json", symbol).unwrap().create();

   // WHEN - we load the data
   block_on(Profile::load(symbol)).expect("failure");

   // THEN - we get an error
}

#[test]
#[should_panic(expected = "CallFailed")]
fn load_bad_response() {
   //! Ensures that we gracefully fail when Yahoo returns an unexpected return code for a symbol

   // GIVEN - a symbol that does not exist
   let symbol = "NULL";
   let _m = base_mock("aapl", symbol).unwrap()
      .with_status(404)
      .create();

   // WHEN - we load the data
   block_on(Profile::load(symbol)).expect("failure");

   // THEN - we get an error
}

#[test]
#[should_panic(expected = "MissingData")]
fn load_missing_data() {
   //! Ensures that we gracefully fail when Yahoo! doesn't return the data we expect

   // GIVEN - a response that has no data
   let symbol = "NULL";
   let _m = base_mock("missing_data", symbol).unwrap().create();

   // WHEN - we load the data
   block_on(Profile::load(symbol)).expect("failure");

   // THEN - we get an error
}
