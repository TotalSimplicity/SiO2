
use crate::db::Db;
use chrono::{DateTime, Utc};


pub struct RawStockData {
    pub ticker: String,
    pub timestamp: DateTime<Utc>,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: i64
}




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