#! [allow(dead_code)]
use chrono::{DateTime, Utc};
use crate::db::Db;

pub struct RawStockData {
    pub ticker: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64,
}

impl RawStockData {
    pub fn get_data_from_db(ticker: &str) -> Result<Vec<RawStockData>, String> {
        let mut db = Db::connect("host=localhost dbname=historicaldata user=admin password=secret")
            .map_err(|e| e.to_string())?;

        let query = "SELECT * FROM hist_stock_data WHERE ticker = $1";
        let rows = db.query_rows(query, &[&ticker])
            .map_err(|e| e.to_string())?;

        let mut stock_data: Vec<RawStockData> = Vec::new();
        for row in rows.iter() {
            let datapoint = RawStockData {
                ticker: row.get("ticker"),
                timestamp: row.get("time"),
                open: row.get::<_, f64>("open"),
                high: row.get::<_, f64>("high"),
                low: row.get::<_, f64>("low"),
                close: row.get::<_, f64>("close"),
                volume: row.get::<_, i64>("volume"),
            };
            stock_data.push(datapoint);
        }

        Ok(stock_data)
    }

    /// Function to extract a specific field from the stock data array as an array of f64 points.
    pub fn get_specific_array(data_arr: Vec<RawStockData>, field: &str) -> Vec<f64> {
        data_arr.iter().map(|data| {
            match field {
                "open" => data.open,
                "high" => data.high,
                "low" => data.low,
                "close" => data.close,
                "volume" => data.volume as f64,
                _ => 0.0,
            }
        }).collect()
    }

    
}





