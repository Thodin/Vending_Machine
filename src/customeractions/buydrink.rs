use super::validateinput::validateinput;
use super::{COCA_COLA, FANTA, PEPSI, SPRITE};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
pub fn buydrink(avail_drinks: &mut HashMap<&str, u32>) -> Result<(), String> {
    let mut drink_option = String::new();
    loop {
        drink_option.clear();

        println!("which drink do you want?");
        println!(
            "1. Coca-Cola : Number of available Cans : {:?}",
            avail_drinks.entry(COCA_COLA).or_insert(0)
        );
        println!(
            "2. Pepsi     : Number of available Cans : {:?}",
            avail_drinks.entry(PEPSI).or_insert(0)
        );
        println!(
            "3. Sprite    : Number of available Cans : {:?}",
            avail_drinks.entry(SPRITE).or_insert(0)
        );
        println!(
            "4. Fanta     : Number of available Cans : {:?}",
            avail_drinks.entry(FANTA).or_insert(0)
        );
        println!("5. Exit");
        let drink_choice = validateinput(&mut drink_option)?;
        match drink_choice {
            1 => reduce_entry(avail_drinks, COCA_COLA),
            2 => reduce_entry(avail_drinks, PEPSI),
            3 => reduce_entry(avail_drinks, SPRITE),
            4 => reduce_entry(avail_drinks, FANTA),
            _ => break,
        }
    }
    Ok(())
}
fn reduce_entry<'a>(avail_drinks: &mut HashMap<&'a str, u32>, drink_name: &'a str) {
    let count = avail_drinks.entry(drink_name).or_default();
    if *count >= 1 {
        println!("Insert the amount in the slot in 50seconds");
        sleep(Duration::new(10, 0));
        *count -= 1;
        println!("Take drink {} from the slot", drink_name);
        sleep(Duration::new(6, 0));
    } else {
        println!("sorry drink is not available");
    }
}
