use crate::VendingMachine;
use crate::customeractions::getprice::getprice;
use crate::loaddrinksprices::Datafilelayout;
use crate::validateinput::validateinput;
pub fn customermenu(
    drinks_prices: &[Datafilelayout],
    vendingmachine: &mut VendingMachine,
) -> Result<(), String> {
    loop {
        println!(
            " Customer Menu\n\
             1. Buy Drink\n\
             2. Exit"
        );
        let user_selected_choice = validateinput()?;
        match user_selected_choice {
            1 => {
                if vendingmachine.inventory.len() > 0 {
                    println!("slot number and correspoding drink");
                    vendingmachine.displayinventory();
                    println!("select the drink by giving slot number ");
                    let selected_slot_number =
                        validateinput().map_err(|_| "unable to read the input")?;
                    if selected_slot_number < vendingmachine.inventory.len() as u32 {
                        // check if selected drink is in stock or not
                        if vendingmachine.instock(selected_slot_number) {
                            // get price for the selected drink
                            let price = get_price_for_the_given_drink(
                                drinks_prices,
                                vendingmachine,
                                selected_slot_number,
                            );

                            if price > 0 {
                                // ask customer to enter the money
                                let result = insert_money_inthe_slot(vendingmachine, price);
                                if let Ok(output) = result {
                                    if output.0 {
                                        // reduce the drink stock by 1
                                        vendingmachine.buydrink(selected_slot_number);
                                        println!("Take drink ,inventory updated");
                                        //update the inventory with the money received from the customer
                                        update_the_inventory(vendingmachine, output.1);
                                    } else {
                                        println!(
                                            " sorry don't have change,please insert exact amount"
                                        );
                                    }
                                }
                            }
                        } else {
                            println!("it doesn't have drink")
                        }
                    } else {
                        println!("invalid slot number")
                    }
                } else {
                    println!("There are no drinks in the machine")
                }
            }

            _ => break,
        }
    }
    Ok(())
}

fn get_price_for_the_given_drink(
    drinks_prices: &[Datafilelayout],
    vendingmachine: &VendingMachine,
    selected_slot_number: u32,
) -> u32 {
    let mut selected_drink_price: u32 = 0;
    let price = getprice(
        drinks_prices,
        &vendingmachine.inventory[selected_slot_number as usize].drinkname,
    );

    if let Ok(x) = price {
        println!("enter {}{} in the slot", x.1, x.0);
        if x.0 == "€" {
            selected_drink_price = x.1 * 100;
        } else {
            selected_drink_price = x.1;
        }
    }
    selected_drink_price
}
fn insert_money_inthe_slot(
    vendingmachine: &VendingMachine,
    price: u32,
) -> Result<(bool, [u32; 7]), String> {
    let price_in_cents = [5, 10, 20, 50, 100, 200, 500];
    let mut inserted_coins_image = [0, 0, 0, 0, 0, 0, 0];
    let _available_coins_in_machine = vendingmachine.available_coins_inv_image();
    let mut inserted_money_value = 0;
    loop {
        println!(
            "0. 5 cent\n1. 10 cent\n2. 20 cent\n3. 50 cent\n4. 1 Eur\n5. 2 Eur\n6. 5 Eur\n7. exit\n "
        );
        let user_selection = validateinput()?;
        if user_selection >= 7 {
            println!("Invalid entry, take the money back ");
            break;
        };
        inserted_coins_image[user_selection as usize] += 1;
        inserted_money_value += price_in_cents[user_selection as usize];
        if inserted_money_value == price {
            return Ok((true, inserted_coins_image));
        }
        if inserted_money_value < price {
            let mut remaining: f32 = (price - inserted_money_value) as f32;

            if remaining >= 100.0 {
                println!(
                    "The remaining amount to be inserted {:?} € ",
                    (remaining / 100.0)
                );
            } else {
                println!("The remaining amount to be inserted {:?} Cents ", remaining);
            }
        }
        if inserted_money_value > price {
            // check if you have enough change, to give it to the customer
            // let change = inserted_money_value - price;
            return Ok((false, inserted_coins_image));
            // can I make the change that need to be given with available image
        }
    }
    Err("invalid result".to_string())
}

fn update_the_inventory(vendingmachine: &mut VendingMachine, inserted_coins_image: [u32; 7]) {
    for item in inserted_coins_image.iter().enumerate() {
        if *item.1 > 0 {
            vendingmachine.addcoins(item.0 as u32, *item.1);
        }
    }
}
