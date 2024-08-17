
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

// use rand::Rng;
// use rust_decimal::Decimal;
// use rust_decimal::{
//     prelude::{FromPrimitive, ToPrimitive},
//     Decimal,
// };
// use rust_decimal_macros::dec;

use crate::stock_market::StockData;

use rand::Rng;
use rust_decimal::Decimal;
use rust_decimal::prelude::FromPrimitive;


#[cfg(test)]
// use super::*;

fn generate_utc_date_from_date_string(date_string: &str) -> DateTime<Utc> {
    let day_one = NaiveDateTime::parse_from_str(date_string, "%m-%d-%Y %H:%M").unwrap();
    Utc.from_utc_datetime(&day_one)
}

fn generate_stock_data(date_string: &str) -> StockData {
    let base_stock_data_series = vec![
        (130.0600, 131.3700, 128.8300, 129.1500),
        (125.7900, 125.8500, 124.5200, 125.0100),
        (124.1000, 125.5800, 123.8300, 125.4400),
        (122.6200, 124.0000, 122.5700, 123.7600),
        (122.1900, 123.5200, 121.3018, 123.3700),
        (121.2400, 121.8500, 120.5400, 121.7700),
        (121.6400, 121.6500, 120.1000, 120.7700),
        (120.9400, 121.5800, 120.5700, 121.0500),
        (120.6400, 120.9800, 120.3700, 120.9500),
        (120.5400, 120.8500, 119.9200, 120.3300),
        (119.7600, 120.3500, 119.5400, 120.1900),
        (118.6300, 119.5400, 118.5800, 119.2800),
        (119.8100, 120.0200, 118.6400, 119.9300),
        (119.3900, 120.2300, 119.3700, 119.8900),
        (120.1000, 120.2300, 118.3800, 119.3600),
        (119.8600, 120.4300, 119.1500, 119.9700),
        (119.0600, 119.4800, 118.5200, 119.1900),
        (118.9500, 119.1085, 118.1000, 119.0200),
        (118.0700, 118.3200, 116.9600, 117.9400),
        (117.4400, 117.5800, 116.1300, 116.9300),
        (117.8750, 118.2100, 115.5215, 116.7700),
        (118.6200, 118.7050, 116.8500, 117.9100),
        (116.5600, 118.0100, 116.3224, 117.6600),
        (119.5000, 119.5900, 117.0400, 117.0500),
        (117.1350, 120.8200, 117.0900, 120.2200),
        (117.3900, 118.7500, 116.7100, 117.5200),
        (118.0900, 118.4400, 116.9900, 117.6500),
        (116.1700, 117.6100, 116.0500, 117.5700),
        (115.3400, 117.2500, 114.5900, 115.9100),
        (114.5400, 115.2000, 114.3300, 114.5900),
    ];

    let base_data_series_len = base_stock_data_series.len();

    let mut rng = rand::thread_rng();

    let high = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
        .unwrap()
        .round_dp(2);
    let low = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
        .unwrap()
        .round_dp(2);
    let open = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].0)
        .unwrap()
        .round_dp(2);
    let close = Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
        .unwrap()
        .round_dp(2);

    StockData::new(
        generate_utc_date_from_date_string(date_string),
        high,
        low,
        open,
        close,
    )
}
