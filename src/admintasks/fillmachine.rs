use super::validateinput::validateinput;
use super::{COCA_COLA, FANTA, PEPSI, SPRITE};
use std::collections::HashMap;

pub fn fillmachine(avail_drinks: &mut HashMap<&str, u32>) -> Result<(), String> {
    let mut drink_option = String::new();

    loop {
        // Could print in one go
        println!("Insert Drinks");
        println!("1. Coca-Cola");
        println!("2. Pepsi");
        println!("3. Sprite");
        println!("4. Fanta");
        println!("5. Exit");

        // Why clear here? We can overwrite a String without clearing.
        drink_option.clear();
        let drink_choice = validateinput(&mut drink_option)?;
        println!("{:?}", drink_choice);

        // So we hardcoded our drinks here. Which means, the code needs to be re-compiled
        // when the operator decides to have some sparkling water in the machine.
        // What I mean by that: you could define more types of drinks than you have slots in the machine.
        // The inventory should only have as many entries as there are slots, but we should give the
        // operator the freedom to choose from a possibly wide array of different drinks.
        // Can you think of a different solution? Maybe re-structure the way you hold your inventory (avail_drinks)?
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
