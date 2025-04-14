use crate::VendingMachine;
// set slots
impl VendingMachine {
    pub fn setslots(&mut self, totalslots: u32) {
        self.slots.totalslots = totalslots;
    }
}
