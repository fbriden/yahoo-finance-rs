use crate::{Interval};
use snafu::{Snafu};

/// All possible errors that can occur when using yahoo finance
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
   #[snafu(display("Start date cannot be after the end date"))]
   InvalidStartDate,

   #[snafu(display("Failed Yahoo! call - {}: {}", code, description))]
   Other { code: String, description: String },

   #[snafu(display("Intraday intervals like {} are not allowed", interval))]
   NoIntraday { interval: Interval },

   #[snafu(display("Symbol '{}' not found", symbol))]
   SymbolNotFound { symbol: &'static str }
}