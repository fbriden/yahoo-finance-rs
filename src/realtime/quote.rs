use chrono::{DateTime, Utc};

use super::data::{ PricingData_MarketHoursType, PricingData_QuoteType };

/// The trading session where a quote has occurred
#[derive(Debug)]
pub enum TradingSession {
   /// The period of trading before the regular market session
   PreMarket,

   /// The period of trading during the regular market session
   Regular,

   /// The period of trading immediately after the regular market session
   AfterHours,

   /// Undefined right now - for future use
   Other
}
impl TradingSession {
   pub(crate) fn from_pd(value: PricingData_MarketHoursType) -> TradingSession {
      match value {
         PricingData_MarketHoursType::PRE_MARKET => TradingSession::PreMarket,
         PricingData_MarketHoursType::REGULAR_MARKET => TradingSession::Regular,
         PricingData_MarketHoursType::POST_MARKET => TradingSession::AfterHours,
         _ => TradingSession::Other
      }
   }
}

/// The type of quote
#[derive(Debug)]
pub enum QuoteType {
   None,
   Equity,
   Index,
   Other
}
impl QuoteType {
   pub(crate) fn from_pd(value: PricingData_QuoteType) -> QuoteType {
      match value {
         PricingData_QuoteType::NONE => QuoteType::None,
         PricingData_QuoteType::EQUITY => QuoteType::Equity,
         PricingData_QuoteType::INDEX => QuoteType::Index,
         _ => QuoteType::Other
      }
   }
}

/// A symbol's quote at a period in time
#[derive(Debug)]
pub struct Quote {
   /// The symbol for the quote
   pub symbol: String,

   /// The type of quote - index, option, etc
   pub quote_type: QuoteType,

   /// The date / time of the quote
   pub timestamp: DateTime<Utc>,

   /// The trading session of the quote - pre market / regular hours / after hours
   pub session: TradingSession,

   /// The price of the quote
   pub price: f32,

   /// The daily volume of the symbol
   pub volume: i64
}
