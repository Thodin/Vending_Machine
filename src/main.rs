mod admintasks;
mod customeractions;
mod validateinput;
use admintasks::fillmachine::fillmachine;
use customeractions::buydrink::buydrink;
use std::collections::HashMap;
use validateinput::validateinput;

pub const COCA_COLA: &str = "Coca Cola";
pub const PEPSI: &str = "Pepsi";
pub const SPRITE: &str = "Sprite";
pub const FANTA: &str = "Fanta";

// General remarks:
// - Try to use empty lines to structure your code a bit better
// - There are several places in the code where you default/insert 0 in your `avail_drinks` if an entry is
//   non-existant. I would prefer if this would be done in one place.
// - You chose a solution that fully avoids structs. I would prefer if you used a struct `VendingMachine`
//   and implemented methods rather than free functions. This would get rid of the handing-around of `avail_drinks`.
// - We are still missing an implementation of the current credit (inserted money) and the change at the end (if any).

fn main() -> Result<(), String> {
    let mut avail_drinks: HashMap<&str, u32> = HashMap::new();
    // unused initialization
    let mut input_option: String;
    loop {
        // Could print this in one statement for better performance (with \n)
        println!("choose an Option");
        println!("1. Maintain the machine");
        println!("2. Choose the drink");
        println!("3. exit");
        input_option = String::new();
        let choice = validateinput(&mut input_option)?;
        let output = match choice {
            1 => fillmachine(&mut avail_drinks),
            2 => buydrink(&mut avail_drinks),
            _ => break,
        };
        if let Err(msg) = output {
            println!(" error message {:?}", msg);
        }
    }
    Ok(())
}
