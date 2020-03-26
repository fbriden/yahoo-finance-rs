use chrono::{DateTime, Utc};

// make sure our macros are all loaded
#[macro_use]
mod macros;

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

// bring in the history code
pub mod history;

// re-export stuff for external use
pub use interval::{Interval};
pub use error::{Error};
