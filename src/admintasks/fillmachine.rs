use super::validateinput::validateinput;
use super::{COCA_COLA, FANTA, PEPSI, SPRITE};
use std::collections::HashMap;
pub fn fillmachine(avail_drinks: &mut HashMap<&str, u32>) -> Result<(), String> {
    let mut drink_option = String::new();

    loop {
        println!("Insert Drinks");
        println!("1. Coca-Cola");
        println!("2. Pepsi");
        println!("3. Sprite");
        println!("4. Fanta");
        println!("5. Exit");
        drink_option.clear();
        let drink_choice = validateinput(&mut drink_option)?;
        println!("{:?}", drink_choice);
        match drink_choice {
            1 => insert_entry(avail_drinks, COCA_COLA),
            2 => insert_entry(avail_drinks, PEPSI),
            3 => insert_entry(avail_drinks, SPRITE),
            4 => insert_entry(avail_drinks, FANTA),
            _ => break,
        }
    }
    Ok(())
}

fn insert_entry<'a>(avail_drinks: &mut HashMap<&'a str, u32>, drink_name: &'a str) {
    let count = avail_drinks.entry(drink_name).or_default();
    *count += 1;
    println!("1 {} added", drink_name);
}
