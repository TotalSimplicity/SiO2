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
   


    let everex_arr = RawStockData::calculate_everex_points(&data_array, 20, 10, 3, 5);

    for point in everex_arr {
        println!("{:?}", point.signal_line);
    }

   

   
}