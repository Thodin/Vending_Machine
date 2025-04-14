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

#[derive(Debug)]
pub struct VendingMachine {
    pub slots: Totalslots,
    pub coinstore: Coinstore,
    pub inventory: Vec<Inventory>,
}
#[derive(Debug)]
pub struct Coinstore {
    pub coins: Vec<Coin>,
}

#[derive(Debug)]
pub struct Totalslots {
    pub totalslots: u32,
}
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
