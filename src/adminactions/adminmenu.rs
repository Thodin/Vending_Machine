use crate::VendingMachine;
use crate::loaddrinksprices::Datafilelayout;
use crate::validateinput::validateinput;
pub fn adminmenu(
    drinks_prices: &Vec<Datafilelayout>,
    vendingmachine: &mut VendingMachine,
) -> Result<(), String> {
    loop {
        println!(
            " Machine Maitanance Menu\n\
             1. Define machine slots\n\
             2. Insert Drink \n\
             3. Display added Drinks\n\
             4. Insert money \n\
             5. Display Cash Inventory \n\
             6. DisplayTotalMoneyValue \n\
             7. Exit"
        );
        let user_selected_choice = validateinput()?;
        match user_selected_choice {
            1 => {
                println!("Enter the total number  of slots");
                let total_slots = validateinput()?;
                vendingmachine.setslots(total_slots);
                println!(
                    "the number of slots set to {:?}",
                    vendingmachine.slots.totalslots
                );
            }
            2 => {
                if (vendingmachine.inventory.len() as u32) < vendingmachine.slots.totalslots {
                    'outer_loop: for i in 0..vendingmachine.slots.totalslots {
                        loop {
                            println!("Choose which drink would like to insert in slot {:?}", i);
                            println!("The available drinks are as follows");
                            for i in drinks_prices {
                                println!("choose {:?} for {:?}", i.item_number, i.drink_name)
                            }
                            println!("choose 99 for EXIT");

                            let user_selection = validateinput()?;
                            if user_selection == 99 {
                                break 'outer_loop;
                            };
                            if user_selection < drinks_prices.len() as u32 {
                                vendingmachine.adddrinks(
                                    &drinks_prices[user_selection as usize].drink_name,
                                    i,
                                );
                                break;
                            } else {
                                println!("Invalid selection, drink not added")
                            }
                        }
                    }
                } else {
                    println!("no free slots to insert the drink");
                }
            }
            3 => {
                if vendingmachine.inventory.len() > 0 {
                    vendingmachine.displayinventory()
                } else {
                    println!("there are no drinks")
                }
            }
            4 => loop {
                println!("Enter the money to be added ");
                println!(
                    "0. 5 cent\n1. 10 cent\n2. 20 cent\n3. 50 cent\n4. 1 Eur\n5. 2 Eur\n6. 5 Eur\n7. exit\n "
                );
                let user_selection = validateinput()?;
                if user_selection >= 7 {
                    break;
                };
                vendingmachine.addcoins(user_selection, 1_u32)
            },
            5 => vendingmachine.displaycoins(),
            6 => println!(
                " the money avaialble in the vending machine {:?}",
                vendingmachine.coinstotalvalue()
            ),
            _ => break,
        }
    }
    Ok(())
}
