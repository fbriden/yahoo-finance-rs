use std::fmt;

/// An interval use when requesting periods of quote information.
/// 
/// Since we cannot start the values with numbers (as they are normally represented),
/// we start them with underscores.
/// 
/// `m` is for minutes. `mo` is for months, the rest should be self explanatory
#[allow(non_camel_case_types)]
#[derive(Debug)]
pub enum Interval { _1m, _2m, _5m, _15m, _30m, _60m, _90m, _1d, _5d, _1mo, _3mo, _6mo, _1y, _2y, _5y, _10y, _ytd, _max }
impl Interval {
   pub fn is_intraday(&self) -> bool {
      match self {
         Interval::_1m | Interval::_2m | Interval::_5m | Interval::_15m | Interval::_30m | Interval::_60m | Interval::_90m => true,
         _ => false
      }
   }
}
impl fmt::Display for Interval {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result  {
      match self {
         Interval::_1m   => write!(f, "1m"),
         Interval::_2m   => write!(f, "2m"),
         Interval::_5m   => write!(f, "5m"),
         Interval::_15m  => write!(f, "15m"),
         Interval::_30m  => write!(f, "30m"),
         Interval::_60m  => write!(f, "60m"),
         Interval::_90m  => write!(f, "90m"),
         Interval::_1d   => write!(f, "1d"),
         Interval::_5d   => write!(f, "5d"),
         Interval::_1mo  => write!(f, "1mo"),
         Interval::_3mo  => write!(f, "3mo"),
         Interval::_6mo  => write!(f, "6mo"),
         Interval::_1y   => write!(f, "1y"),
         Interval::_2y   => write!(f, "2y"),
         Interval::_5y   => write!(f, "5y"),
         Interval::_10y  => write!(f, "10y"),
         Interval::_ytd  => write!(f, "ytd"),
         Interval::_max  => write!(f, "max")
      }
   }
}
