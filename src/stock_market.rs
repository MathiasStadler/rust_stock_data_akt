// use std::{error::Error, fs};

// use chrono::{prelude::*, Duration};
// use plotters::{
//     prelude::{
//         BitMapBackend, CandleStick, ChartBuilder, IntoDrawingArea, PathElement, SeriesLabelPosition,
//     },
//     series::LineSeries,
//     style::{
//         full_palette::{ORANGE, PURPLE},
//         Color, IntoFont, RGBColor, BLUE, GREEN, RED, WHITE,
//     },
// };
use rust_decimal::Decimal;
use chrono::Utc;
use chrono::DateTime;
// use rust_decimal_macros::dec;



// net_change and net_change_percent will be computed.
#[derive(Debug)]
pub struct StockData {
    date: DateTime<Utc>,
    high: Decimal,
    low: Decimal,
    open: Decimal,
    close: Decimal,
    net_change: Option<Decimal>,
    net_change_percent: Option<Decimal>,
}

impl StockData {
    pub fn new(
        date: DateTime<Utc>,
        high: Decimal,
        low: Decimal,
        open: Decimal,
        close: Decimal,
    ) -> Self {
        Self {
            date,
            high,
            low,
            open,
            close,
            net_change: None,
            net_change_percent: None,
        }
    }
}