use chrono::{NaiveDate, NaiveDateTime};
/*
    Retrieval of time series market data from Alphavantage (https://www.alphavantage.co/documentation/)
*/
use reqwest::get;
use serde::{de, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::{
    collections::HashMap,
    error::Error,
    fmt::{Display, Formatter},
};

const BASE_URL: &str = "https://www.alphavantage.co/query";
const API_KEY: &str = "API_KEY";

#[tokio::test]
async fn test_read_time_series() -> Result<(), Box<dyn Error>> {
    let _ = TimeSeries::read_time_series("data/ibm_time_series_daily.json").await?;
    let ts = TimeSeries::read_time_series("data/ibm_time_series_intraday.json").await?;
    println!("{:?}", ts);
    Ok(())
}

// #[tokio::test]
async fn test_http_get_time_series() -> Result<(), Box<dyn Error>> {
    let ts = TimeSeries::http_get_time_series("AAPL", TimeSeriesType::Daily).await?;
    println!("{:?}", ts);
    Ok(())
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

#[derive(Deserialize, Debug)]
pub struct TimeSeries {
    #[serde(rename = "Meta Data")]
    meta_data: MetaData,
    #[serde(flatten)]
    #[serde(deserialize_with = "de_time_series")]
    time_series: HashMap<NaiveDateTime, TimeSeriesEntry>,
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
    #[serde(deserialize_with = "de_f32", rename = "1. open")]
    open: f32,
    #[serde(deserialize_with = "de_f32", rename = "2. high")]
    high: f32,
    #[serde(deserialize_with = "de_f32", rename = "3. low")]
    low: f32,
    #[serde(deserialize_with = "de_f32", rename = "4. close")]
    close: f32,
    #[serde(deserialize_with = "de_u32", rename = "5. volume")]
    volume: u32,
}

impl TimeSeries {
    pub async fn http_get_time_series(
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

        let body: String = get(&url).await?.text().await?;
        let ts: TimeSeries = serde_json::from_str(&body)?;
        Ok(ts)
    }

    pub async fn read_time_series(path: &str) -> Result<TimeSeries, Box<dyn std::error::Error>> {
        let content: String = tokio::fs::read_to_string(path).await?;
        let ts: TimeSeries = serde_json::from_str(&content)?;
        Ok(ts)
    }

    pub fn high(&self) -> HashMap<NaiveDateTime, f32> {
        self.time_series
            .iter()
            .map(|(datetime, entry)| (*datetime, entry.high))
            .collect()
    }
    // pub fn
}

fn de_time_series<'de, D>(
    deserializer: D,
) -> Result<HashMap<NaiveDateTime, TimeSeriesEntry>, D::Error>
where
    D: Deserializer<'de>,
{
    // deserialize to a single-entry HashMap (whose key is dynamic) and extract the only value
    let map: HashMap<String, HashMap<String, TimeSeriesEntry>> =
        HashMap::deserialize(deserializer)?;

    let m = map
        .into_iter()
        .next()
        .map(|(_, value)| value)
        .ok_or_else(|| serde::de::Error::custom("Missing time series data"))?;

    // parse date or datetime into NaiveDateTime
    m.into_iter()
        .map(|(k, v)| {
            // %Y-%m-%d
            if let Ok(datetime) = NaiveDate::parse_from_str(&k, "%Y-%m-%d") {
                Ok(datetime
                    .and_hms_opt(0, 0, 0)
                    .map(|time| ((time, v)))
                    .unwrap())
            }
            // %Y-%m-%d %H:%M:%S
            else {
                NaiveDateTime::parse_from_str(&k, "%Y-%m-%d %H:%M:%S")
                    .map(|time| ((time, v)))
                    .map_err(|_| {
                        de::Error::custom(format!(
                            "Failed to parse from both %Y-%m-%d and %Y-%m-%d %H:%M:%S:{}",
                            k
                        ))
                    })
            }
        })
        .collect() // collect() collapses an iterator of Result<(k, v), _> into a Result<HashMap<k, v>, _>.
}

fn de_f32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<f32, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_f64().ok_or(de::Error::custom("Invalid number"))? as f32,
        _ => return Err(de::Error::custom("wrong type")),
    })
}

fn de_u32<'de, D: Deserializer<'de>>(deserializer: D) -> Result<u32, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s.parse().map_err(de::Error::custom)?,
        Value::Number(num) => num.as_u64().ok_or(de::Error::custom("Invalid number"))? as u32,
        _ => return Err(de::Error::custom("wrong type")),
    })
}
