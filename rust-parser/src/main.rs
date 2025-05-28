mod stockdata;
mod db;
use stockdata::get_data_from_db;
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
    
   for datapoint in get_data_from_db(&ticker).unwrap(){
    println!("Ticker: {}, Time: {}, Open: {}, High: {}, Low: {}, Close: {}, Volume: {}",
             datapoint.ticker,
             datapoint.timestamp.to_rfc3339(),
             datapoint.open,
             datapoint.high,
             datapoint.low,
             datapoint.close,
             datapoint.volume);
   }

}