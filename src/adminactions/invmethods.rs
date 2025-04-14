use crate::Inventory;
use crate::VendingMachine;

impl VendingMachine {
    //add drinks
    pub fn adddrinks(&mut self, drinkname: &str, slotnumber: u32) {
        let itemdetails = Inventory {
            slotnumber,
            drinkname: drinkname.to_string(),
            slotsize: 5,
        };
        self.inventory.push(itemdetails);
    }

    pub fn displayinventory(&self) {
        //display avaialabe drinks and their slot number
        for i in &self.inventory {
            println!(
                "slot number {} contains {} drink Quantity {}",
                i.slotnumber, i.drinkname, i.slotsize
            )
        }
    }
    //is the given slot empty
    pub fn instock(&self, slotnumber: u32) -> bool {
        self.inventory[slotnumber as usize].slotsize > 0
    }
}
