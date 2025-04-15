use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum NewCoin {
    FiveCents,
    TenCents,
    TwentyCents,
    FiftyCents,
    OneEuro,
    TwoEuro,
}

// MM: I think this would be a very helpful method.
impl NewCoin {
    pub fn value_in_cents(&self) -> u32 {
        match self {
            NewCoin::FiveCents => 5,
            NewCoin::TenCents => 10,
            NewCoin::TwentyCents => 20,
            NewCoin::FiftyCents => 50,
            NewCoin::OneEuro => 100,
            NewCoin::TwoEuro => 200,
        }
    }
}

pub struct CoinStore {
    pub coins: HashMap<NewCoin, u32>,
}

impl CoinStore {
    pub fn new() -> Self {
        let mut coins = HashMap::new();
        coins.insert(NewCoin::FiveCents, 0);
        coins.insert(NewCoin::TenCents, 0);
        coins.insert(NewCoin::TwentyCents, 0);
        coins.insert(NewCoin::FiftyCents, 0);
        coins.insert(NewCoin::OneEuro, 0);
        coins.insert(NewCoin::TwoEuro, 0);

        CoinStore { coins }
    }

    pub fn add_coin(&mut self, coin: NewCoin) {
        self.coins
            .entry(coin)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    pub fn total_value(&self) -> u32 {
        self.coins
            .iter()
            .map(|(coin, &count)| coin.value_in_cents() * count)
            .sum()
    }
}
