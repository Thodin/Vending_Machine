use std::fs::File;
use std::io::{BufRead, BufReader};
pub struct Datafilelayout {
    pub item_number: u32,
    pub drink_name: String,
    pub currency_symbol: String,
    pub price: u32,
}
// This module is responsible for loading data from the file `data.txt`.
// The file contains information about drinks and their prices.
// The file header includes: ITEMNUMBER, DRINKNAME, CURRENCY_SYMBOL, PRICE.

pub fn load_drinks_and_their_prices() -> Result<Vec<Datafilelayout>, String> {
    let mut drinks_and_prices: Vec<Datafilelayout> = Vec::new();
    // Open the file
    let file = File::open("./src/drinks.txt").map_err(|_| "Unable to open the file".to_string())?;
    //  attach Buffer reader to the file
    let reader = BufReader::new(file);
    // read file line by line
    let mut datafilelayout: Datafilelayout;
    for line_result in reader.lines().skip(1) {
        let line = line_result.map_err(|_| "XXX".to_string())?;
        let temp: Vec<&str> = line.split(",").collect();

        if temp.len() == 4 {
            datafilelayout = Datafilelayout {
                item_number: temp[0].trim().parse::<u32>().map_err(|_| " ".to_string())?,
                drink_name: temp[1].trim().to_string(),
                currency_symbol: temp[2].trim().to_string(),
                price: temp[3].trim().parse::<u32>().map_err(|_| " ".to_string())?,
            };
            drinks_and_prices.push(datafilelayout);
        } else {
            println!("row ignored due to length mismatch{:?}", temp);
        }
    }
    Ok(drinks_and_prices)
}
