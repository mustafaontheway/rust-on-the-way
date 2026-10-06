fn main() {

    let profit_or_loss = calculate_profit(750_000, 440_000);

    if profit_or_loss.is_ok() {

        println!("Awesome!")
    } 

    if profit_or_loss.is_err() {

        println!("Try, try, try...")
    }
}

fn calculate_profit(sales: u64, cost: u64) -> Result<u64, String> {

    if sales >= cost {

        Ok(sales - cost)
    }

    else {
        
        Err(format!("Cost amount ({cost} ₺) exceeds sales amount ({sales} ₺)!"))
    }
}

// Awesome!
