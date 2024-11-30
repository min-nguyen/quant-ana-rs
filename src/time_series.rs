/*
    Retrieval of time series market data from Alphavantage (https://www.alphavantage.co/documentation/)
*/
use reqwest::get;
use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
};

const BASE_URL: &str = "https://www.alphavantage.co/query";
const API_KEY: &str = "API_KEY";

#[tokio::test]
async fn test_get_time_series() -> Result<(), Box<dyn Error>> {
    let ts = TimeSeries::get_time_series("AAPL", TimeSeriesType::Daily).await?;
    println!("{:?}", ts);
    Ok(())
    // TimeSeries::get_time_series("AAPL", TimeSeriesType::DailyAdjusted).await?;
    // TimeSeries::get_time_series("AAPL", TimeSeriesType::Weekly).await?;
    // TimeSeries::get_time_series("AAPL", TimeSeriesType::WeeklyAdjusted).await?;
    // TimeSeries::get_time_series("AAPL", TimeSeriesType::Monthly).await?;
    // TimeSeries::get_time_series("AAPL", TimeSeriesType::MonthlyAdjusted).await
}

/* ----------------------------------------------------------- */

pub enum TimeSeriesType {
    Intraday(Interval),
    Daily,
    DailyAdjusted,
    Weekly,
    WeeklyAdjusted,
    Monthly,
    MonthlyAdjusted,
}

impl Display for TimeSeriesType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use TimeSeriesType::*;
        match self {
            Intraday(..) => f.write_str("INTRADAY"),
            Daily => f.write_str("DAILY"),
            DailyAdjusted => f.write_str("DAILY_ADJUSTED"),
            Weekly => f.write_str("WEEKLY"),
            WeeklyAdjusted => f.write_str("WEEKLY_ADJUSTED"),
            Monthly => f.write_str("MONTHLY"),
            MonthlyAdjusted => f.write_str("MONTHLY_ADJUSTED"),
        }
    }
}
pub enum Interval {
    _1MIN,
    _5MIN,
    _15MIN,
    _30MIN,
    _60MIN,
}
impl Display for Interval {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::_1MIN => f.write_str("1min"),
            Interval::_5MIN => f.write_str("5min"),
            Interval::_15MIN => f.write_str("15min"),
            Interval::_30MIN => f.write_str("30min"),
            Interval::_60MIN => f.write_str("60min"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TimeSeries {
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(flatten)]
    #[serde(deserialize_with = "deserialize_time_series")]
    time_series: HashMap<String, TimeSeriesEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
struct MetaData {
    #[serde(rename = "1. Information")]
    information: String,
    #[serde(rename = "2. Symbol")]
    symbol: String,
    #[serde(rename = "3. Last Refreshed")]
    last_refreshed: String,
    #[serde(rename = "4. Interval", default)]
    interval: Option<String>,
    #[serde(alias = "4. Output Size", alias = "5. Output Size")]
    output_size: String,
    #[serde(alias = "5. Time Zone", alias = "6. Time Zone")]
    time_zone: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct TimeSeriesEntry {
    #[serde(rename = "1. open")]
    open: String,
    #[serde(rename = "2. high")]
    high: String,
    #[serde(rename = "3. low")]
    low: String,
    #[serde(rename = "4. close")]
    close: String,
    #[serde(rename = "5. volume")]
    volume: String,
}

impl TimeSeries {
    async fn get_time_series(
        symbol: &str,
        ts: TimeSeriesType,
    ) -> Result<TimeSeries, Box<dyn Error>> {
        let url: String = {
            let mut url: String = format!(
                "{}?function=TIME_SERIES_{}&symbol={}&apikey={}",
                BASE_URL, ts, symbol, API_KEY
            );
            if let TimeSeriesType::Intraday(interval) = ts {
                url.push_str(&format!("&interval={}", interval))
            }
            url
        };

        let rsp_body: String = get(&url).await?.text().await?;

        let data: TimeSeries = serde_json::from_str(&rsp_body)?;
        Ok(data)
    }
}

fn deserialize_time_series<'de, D>(
    deserializer: D,
) -> Result<HashMap<String, TimeSeriesEntry>, D::Error>
where
    D: Deserializer<'de>,
{
    /* Deserialize to a single-entry HashMap (whose key is dynamic) and extract the only value */
    let map: HashMap<String, HashMap<String, TimeSeriesEntry>> =
        HashMap::deserialize(deserializer)?;

    map.into_iter()
        .next()
        .map(|(_, value)| value)
        .ok_or_else(|| serde::de::Error::custom("Missing time series data"))
}
