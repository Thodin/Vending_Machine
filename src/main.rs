// validate input modules
mod validateinput;
use validateinput::validateinput;

//  load drink prices and avaialable drinks from data.txtfile
mod loaddrinksprices;
use loaddrinksprices::load_drinks_and_their_prices;

// Admin menu
mod adminactions;
use adminactions::adminmenu::adminmenu;

// customer related modules
mod customeractions;
use customeractions::customermenu::customermenu;

mod coin_store;

// MM: Quite a lot going on in the main file, maybe we want some of these structs in their respective
// modules?
#[derive(Debug)]
pub struct VendingMachine {
    pub slots: Totalslots,
    pub coinstore: Coinstore,
    pub inventory: Vec<Inventory>,
}

// MM:
#[derive(Debug)]
pub struct Coinstore {
    pub coins: Vec<Coin>,
}

#[derive(Debug)]
pub struct Totalslots {
    pub totalslots: u32,
}

// MM: The enum for the coins is great, but I think the u32 doesn't really make sense here.
// The problem is that it's not a fixed value per coin, but a variable value that can change.
// You use it in your coin to represent the total value held in that coin type,
// but that seems a bit odd.
// So I would remove the u32 from the enum and just use it as a simple enum to represent the coin type.
// Maybe encode the value in a method of the CoinStore, which matches on the coin type and returns the value?
// Also, it is convention to use PascalCase for enum variants, so `FiveCents` instead of `FIVECENTS`.
#[derive(Debug)]
pub enum Coin {
    FIVECENTS(u32),
    TENCENTS(u32),
    TWENTYCENTS(u32),
    FIFTYCENTS(u32),
    ONEEURO(u32),
    TWOEURO(u32),
    FIVEEURO(u32),
}

#[derive(Debug)]
pub struct Inventory {
    pub slotnumber: u32,
    pub drinkname: String,
    pub slotsize: u32,
}

// MM: I think we could use anyhow for error management,
// it is a bit more ergonomic than the standard library.
fn main() -> Result<(), String> {
    //iniliaze vending machine
    let mut vendingmachine: VendingMachine = VendingMachine::new();
    //Load Available drinks and their prices from file
    let drinks_prices = load_drinks_and_their_prices()?;
    // start main logic
    loop {
        println!("choose a Role");
        println!("1. Admin ");
        println!("2. Customer");
        println!("3. exit");
        let user_selection = validateinput()?;
        let _output = match user_selection {
            1 => adminmenu(&drinks_prices, &mut vendingmachine),
            2 => customermenu(&drinks_prices, &mut vendingmachine),
            _ => break,
        };
    }
    Ok(())
}

impl VendingMachine {
    // Initialize vending machine
    fn new() -> Self {
        Self {
            slots: Totalslots { totalslots: 0 },
            coinstore: Coinstore {
                coins: initialize_coinstore(),
            },
            inventory: Vec::new(),
        }
    }
}

// MM: This is a very interesting way of holding the coins in the store, but not what I would expect.
// I think there's more idiomatic ways to do this:
// - You could use a HashMap to hold the coins, with the coin type as the key and the value as the amount of that coin type.
// - Or, you could use a Vec<Coin> and just push new coins as they come in.
// The HashMap makes returning the change easier (as you see exactly, what is in the store),
// while the Vec<Coin> makes it easier to add coins to the store (as you just push them). Your decision :)
fn initialize_coinstore() -> Vec<Coin> {
    let x: Vec<Coin> = vec![
        Coin::FIVECENTS(0),
        Coin::TENCENTS(0),
        Coin::TWENTYCENTS(0),
        Coin::FIFTYCENTS(0),
        Coin::ONEEURO(0),
        Coin::TWOEURO(0),
        Coin::FIVEEURO(0),
    ];
    x
}
