use crate::loaddrinksprices::Datafilelayout;
pub fn getprice(
    drinks_prices: &[Datafilelayout],
    drink_name: &str,
) -> Result<(String, u32), String> {
    let price = drinks_prices.iter().find(|x| x.drink_name == drink_name);

    if let Some(x) = price {
        Ok((x.currency_symbol.clone(), x.price))
    } else {
        Err("invalid Price".to_string())
    }
}
