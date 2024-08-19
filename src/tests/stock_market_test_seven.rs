mod tests {

    use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};

    use rand::Rng;
    use rust_decimal::prelude::FromPrimitive;
    use rust_decimal::Decimal;

    // ta
    use ta::indicators::SimpleMovingAverage as Sma;
    use ta::DataItem;
    use ta::Next;

    // for csv read from file
    // use serde::Deserialize;

    #[cfg(test)]
    use rust_stock_data_akt::stock_market::StockData;
    use rust_stock_data_akt::stock_market::StockInformation;

    const CSV_STOCK_INPUT: &str = "stock_data/stock_trex_data.csv";

    // #[derive(Debug, Deserialize)]
    // struct Record {
    //     #[serde(rename = "Date")]
    //     date: String,

    //     #[serde(rename = "Open")]
    //     open: f32,

    //     #[serde(rename = "High")]
    //     high: f32,

    //     #[serde(rename = "Low")]
    //     low: f32,

    //     #[serde(rename = "Close")]
    //     close: f32,

    //     #[serde(rename = "Volume")]
    //     volume: f32,
    // }

    fn generate_utc_date_from_date_string(date_string: &str) -> DateTime<Utc> {
        let day_one = NaiveDateTime::parse_from_str(date_string, "%m-%d-%Y %H:%M").unwrap();
        Utc.from_utc_datetime(&day_one)
    }

    #[allow(dead_code)]
    fn generate_stock_data_from_csv(date_string: &str) -> StockData {
        
        let mut sma = Sma::new(7).unwrap();
        let mut reader = csv::Reader::from_path(CSV_STOCK_INPUT).unwrap();

        let mut vec_stock_data:Vec<(String, f64, f64, f64, f64, f64)> = Vec::new(); // ("String",1,2,3,4,5);
        println!("{}",vec_stock_data.len());



        // base_stock_data_series.push();

        for record in reader.deserialize() {
            let (_date, open, high, low, close, volume): (String, f64, f64, f64, f64, f64) =
                record.unwrap();

            let dt = DataItem::builder()
                .open(open)
                .high(high)
                .low(low)
                .close(close)
                .volume(volume)
                .build()
                .unwrap();

               // vec_stock_data.push([_date,open,high,low,close,volume]);

               // println!("{}",  vec_stock_data.len());

            let _sma_val = sma.next(&dt);
            // println!("{}: {} = {:2.2}", date, sma, sma_val);
            
            //
            // println!(
            //     " {:?}, {:?}, {:?}, {:?},{:?}, {:?}, {:2.2}",
            //     _date, open, high, low, close, volume, _sma_val
            // );
            //
            // println!("{}: {} = {:2.2}", date, sma, sma_val);
        }

        // let base_stock_data_series: Vec<_> = vec![];
        // let base_stock_data_series: Vec<(f64, f64, f64, f64)> = vec![];
        // let base_data_series_len = base_stock_data_series.len();

        let mut rng = rand::thread_rng();

        let high =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
                .unwrap()
                .round_dp(2);
        let low =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
                .unwrap()
                .round_dp(2);
        let open =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].0)
                .unwrap()
                .round_dp(2);
        let close =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
                .unwrap()
                .round_dp(2);

        StockData::new(
            generate_utc_date_from_date_string(date_string),
            high,
            low,
            open,
            close,
        )

        // 
    }

    #[test]
    fn test_generate_stock_data_from_csv() {

        let stock_date:&str = "" ;

        let _data = generate_stock_data_from_csv(&stock_date);

        assert!(true);
        
    }

    // cargo test--package rust_stock_data_akt --bin rust_stock_data_akt -- tests::stock_market_test_seven::tests::test_generate_stock_data_from_csv
    // --exact --show-output

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

        let high =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].1)
                .unwrap()
                .round_dp(2);
        let low =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].2)
                .unwrap()
                .round_dp(2);
        let open =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].0)
                .unwrap()
                .round_dp(2);
        let close =
            Decimal::from_f64(base_stock_data_series[rng.gen_range(0..base_data_series_len)].3)
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

    fn generate_stock_data_series(limit: Option<u8>) -> Vec<StockData> {
        let mut stock_data_series: Vec<StockData> = vec![];
        for number in 0..limit.unwrap_or(7) {
            let number_plus = number + 1;

            let stock_date = match number_plus {
                number_plus if number_plus >= 10 => format!("10-{number_plus}-2022 00:00"),
                _ => format!("10-0{number_plus}-2022 00:00"),
            };

            let stock_data = generate_stock_data(&stock_date);
            stock_data_series.push(stock_data);
        }
        stock_data_series
    }

    // chart output
    #[test]
    fn it_creates_a_new_stock_information_with_data_series_and_show_chart_with_moving_average() {
        let stock_data_series = generate_stock_data_series(Some(14));
        let stock_information = StockInformation::new(
            "BenCorpo".to_string(),
            "BNCRP".to_string(),
            stock_data_series,
        );

        let ma_days = vec![12, 2, 0];
        let chart = stock_information.show_chart(ma_days, None, None, None);

        match chart {
            Ok(_) => {
                assert!(true)
            }
            Err(err) => println!("Error in saving chart {:?}", err),
        }
    }
}

//cargo test stock_market_trex_test
