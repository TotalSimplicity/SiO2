mod stockdata;
mod db;
mod averaging;
mod everex;
use stockdata::RawStockData;
use std::env;




fn handle_ticker_arg() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        return args[1].to_uppercase();
    }
    "XAUUSD".to_string()
}

fn main() {
    let ticker = handle_ticker_arg();
    
    let data_array = RawStockData::get_data_from_db(&ticker).unwrap();
   
    let volume_arr = RawStockData::get_specific_array(data_array, "volume");

    let lookback_len = 20;

   

   
}