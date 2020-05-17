use reqwest::Url;
use serde::Deserialize;
use snafu::{ ensure, OptionExt, ResultExt };
use std::env;
use std::io::{ BufRead, Cursor };

use crate::{ error, Result };

const DATA_VAR: &'static str = "root.App.main";

const BASE_URL: &'static str = "https://finance.yahoo.com";

ez_serde!(QuoteType {
   #[serde(rename = "longName")] name: String,
   #[serde(rename = "quoteType")] kind: String
});

ez_serde!(CompanyProfile {
   address1: Option<String>,
   address2: Option<String>,
   city: Option<String>,
   state: Option<String>,
   country: Option<String>,
   zip: Option<String>,

   #[serde(rename = "fullTimeEmployees")] employees: Option<u32>,

   sector: Option<String>,
   industry: Option<String>,

   #[serde(rename = "longBusinessSummary")] summary: Option<String>,

   website: Option<String>
});

ez_serde!(FundProfile {
   #[serde(rename = "legalType")] kind: String,

   family: Option<String>
});

ez_serde!(QuoteSummaryStore {
   #[serde(rename = "fundProfile")] fund_profile: Option<FundProfile>,
   #[serde(rename = "summaryProfile")] company_profile: Option<CompanyProfile>,
   #[serde(rename = "quoteType")] quote_type: QuoteType
});
ez_serde!(Stores { #[serde(rename = "QuoteSummaryStore")] quote_summary_store: QuoteSummaryStore });
ez_serde!(Dispatcher { stores: Stores });
ez_serde!(Context { dispatcher: Dispatcher });
ez_serde!(Response { context: Context });

pub async fn scrape<'a>(symbol: &'a str) -> Result<Stores> {
   // construct the lookup URL - encoding it so we're safe
   let base = format!("{}/quote/{}", env::var("TEST_URL").unwrap_or(BASE_URL.to_string()), symbol);

   let mut url = Url::parse(base.as_str()).context(error::InternalURL { url: base })?;
   url.query_pairs_mut().append_pair("p", symbol);

   // make the call - we do not really expect this to fail.
   // ie - we won't 404 if the symbol doesn't exist
   let response = reqwest::get(url.clone()).await.context(error::RequestFailed)?;
   ensure!(
      response.status().is_success(),
      error::CallFailed{ url: response.url().to_string(), status: response.status().as_u16() }
   );

   let line = Cursor::new(response.text().await.context(error::UnexpectedErrorRead { url: url.clone().to_string() })?)
      .lines()
      .map(|line| line.unwrap())
      .filter(|line| line.trim().starts_with(DATA_VAR))
      .next()
      .context(error::MissingData { reason: "no quote data" })?;
   
   let data = line
      .trim()
      .trim_start_matches(DATA_VAR)
      .trim_start_matches(|c| c == ' ' || c == '=')
      .trim_end_matches(';');

   let response = serde_json::from_str::<Response>(data).context(error::BadData)?;
   Ok(response.context.dispatcher.stores)
}
