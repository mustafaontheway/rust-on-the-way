fn main() {

    let profit_or_loss = calculate_profit(750_000, 440_000);

    println!("Sales result 1: {profit_or_loss:?}");

    let profit_or_loss = calculate_profit(350_000, 440_000);

    println!("Sales result 2: {profit_or_loss:?}");
}

fn calculate_profit(sales: u64, cost: u64) -> Result<u64, String> {

    if sales >= cost {

        Ok(sales - cost)
    }

    else {
        
        Err(format!("Cost amount ({cost} ₺) exceeds sales amount ({sales} ₺)!"))
    }
}

// Sales result 1: Ok(310000)
// Sales result 2: Err("Cost amount (440000 ₺) exceeds sales amount (350000 ₺)!")
