use chrono::{Duration, Utc};
use yahoo_finance::{ history, Error, Interval};

#[test]
fn invalid_interval() -> Result<(), Error> {
   // GIVEN a stock
   // WHEN we get an intraday history
   match history::retrieve_interval("AAPL", Interval::_1m) {
      Ok(_data) => panic!("1m intervals should NOT be allowed"),

      // THEN we fail
      Err(_message) => Ok(())
   }
}

#[test]
fn valid_range_no_end() -> Result<(), Error> {
   // GIVEN a stock
   // WHEN we get a date range where the start date is after the end date
   match history::retrieve_range("AAPL", Utc::now() + Duration::days(10), None) {
      Ok(_data) => panic!("start date cannot be after end date"),

      // THEN we fail
      Err(_message) => Ok(())
   }
}

#[test]
fn invalid_range() -> Result<(), Error> {
   // GIVEN a stock
   // WHEN we get a date range where the start date is after the end date
   match history::retrieve_range("AAPL", Utc::now() - Duration::days(10), Some(Utc::now() - Duration::days(15))) {
      Ok(_data) => panic!("start date cannot be after end date"),

      // THEN we fail
      Err(_message) => Ok(())
   }
}