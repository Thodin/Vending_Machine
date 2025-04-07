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
fn main() -> Result<(), String> {
    let mut avail_drinks: HashMap<&str, u32> = HashMap::new();
    let mut input_option: String = String::new();
    loop {
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
