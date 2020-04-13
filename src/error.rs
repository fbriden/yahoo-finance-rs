use crate::Interval;
use reqwest;
use snafu::Snafu;

/// All possible errors that can occur when using yahoo finance
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub(crate)")]
pub enum InnerError {
   #[snafu(display("Yahoo! returned invalid data - {}", source.to_string()))]
   BadData { source: serde_json::Error },

   #[snafu(display("Yahoo! call failed. '{}' returned a {} result.", url, status))]
   CallFailed { url: String, status: u16 },

   #[snafu(display("Yahoo! chart failed to load {} - {}.", code, description))]
   ChartFailed { code: String, description: String },

   #[snafu(display("An internal error occurred - please report that '{}'", reason))]
   InternalLogic { reason: String },

   #[snafu(display("An internal error occurred - please report that '{}' cannot be parsed because {}", url, source.to_string()))]
   InternalURL { url: String, source: url::ParseError },

   #[snafu(display("Start date cannot be after the end date"))]
   InvalidStartDate,

   #[snafu(display("Yahoo! returned invalid data - {}", reason))]
   MissingData { reason: String },

   #[snafu(display("Intraday intervals like {} are not allowed", interval))]
   NoIntraday { interval: Interval },

   #[snafu(display("Yahoo! call failed for unknown reason."))]
   RequestFailed { source: reqwest::Error },

   #[snafu(display("Unexpected Yahoo! failure. '{}' returned a {}", url, code))]
   UnexectedFailure { url: String, code: u16 },

   #[snafu(display("Unexpected error while reading data from '{}'", url))]
   UnexpectedErrorRead { url: String, source: reqwest::Error },

   #[snafu(display("Yahoo! call failed.  Expected data is missing."))]
   UnexpectedErrorYahoo,

   #[snafu(display("Unexpected error from Yahoo! - data missing"))]
   Unknown,

   #[snafu(display("We currently do not support securities of type '{}'", kind))]
   UnsupportedSecurity { kind: String }
}