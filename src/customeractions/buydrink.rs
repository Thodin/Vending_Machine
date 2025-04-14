use crate::VendingMachine;
impl VendingMachine {
    pub fn buydrink(&mut self, slotnumber: u32) {
        self.inventory[slotnumber as usize].slotsize -= 1;
    }
}
