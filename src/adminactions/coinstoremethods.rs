use crate::Coin;
use crate::VendingMachine;

// MM: I would definitely prefer if most of this functionality was implemented on `CoinStore`
// instead of `VendingMachine` directly. It would be more modular and easier to test.
impl VendingMachine {
    // add coins to the inventory
    // MM: Probably don't need a `user_selection` here, as the user doesn't need to confirm
    // on the number pad what coin was inserted. Instead, imagine that this function gets triggered
    // by some hardware module that could figure out what coin it is.
    pub fn addcoins(&mut self, user_selection: u32, inc_value: u32) {
        //
        for item in &mut self.coinstore.coins {
            match (user_selection, item) {
                (0, Coin::FIVECENTS(value)) => *value += inc_value,
                (1, Coin::TENCENTS(value)) => *value += inc_value,
                (2, Coin::TWENTYCENTS(value)) => *value += inc_value,
                (3, Coin::FIFTYCENTS(value)) => *value += inc_value,
                (4, Coin::ONEEURO(value)) => *value += inc_value,
                (5, Coin::TWOEURO(value)) => *value += inc_value,
                (6, Coin::FIVEEURO(value)) => *value += inc_value,
                (_, _) => (),
            }
        }
    }
    // display coins to the inventory
    pub fn displaycoins(&self) {
        println!("{:?}", self.coinstore.coins);
    }
    // send back available coins image
    // MM: This method seems a bit odd: It's only used once, and even then the return value is not used.
    // What is the purpose of this function? Wouldn't it be enough that the `coins` vec is already public?
    pub fn available_coins_inv_image(&self) -> [u32; 7] {
        let mut available_coins_image = [0, 0, 0, 0, 0, 0, 0];
        for (index, item) in self.coinstore.coins.iter().enumerate() {
            match item {
                Coin::FIVECENTS(value)
                | Coin::TENCENTS(value)
                | Coin::TWENTYCENTS(value)
                | Coin::FIFTYCENTS(value)
                | Coin::ONEEURO(value)
                | Coin::TWOEURO(value)
                | Coin::FIVEEURO(value) => available_coins_image[index] = *value,
            }
        }
        available_coins_image
    }
    // total coins amount
    pub fn coinstotalvalue(&self) -> f32 {
        let mut total: f32 = 0.0;
        // MM: I would move this logic into a separate method on the `Coin` enum.
        for item in &self.coinstore.coins {
            match item {
                Coin::FIVECENTS(value) => total += *value as f32 * 5.0,
                Coin::TENCENTS(value) => total += *value as f32 * 10.0,
                Coin::TWENTYCENTS(value) => total += *value as f32 * 20.0,
                Coin::FIFTYCENTS(value) => total += *value as f32 * 50.0,
                Coin::ONEEURO(value) => total += *value as f32 * 100.0,
                Coin::TWOEURO(value) => total += *value as f32 * 200.0,
                Coin::FIVEEURO(value) => total += *value as f32 * 500.0,
            }
        }

        // MM: I still find this very confusing. Why do we divide by 100.0 here? Is it because the value is in cents?
        if total > 0.0 {
            total /= 100.0
        };
        total
    }
}
