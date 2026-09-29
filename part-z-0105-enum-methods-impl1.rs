fn main() {
  
    let mounter_strike = Products::GameMounterStrike {
        price: 4.21,
        sales_amount: 37_000,
    };

    let sales_result_2025 = mounter_strike.calculate_profit_or_loss();

    println!(
        "Profit or loss for Mounter Strike in 2025: {} ₺.",
        sales_result_2025
    );
}

#[derive(Debug)]
enum Products {
    GameNario { price: f32, sales_amount: u64 },
    GameMounterStrike { price: f32, sales_amount: u64 },
    FinTechBethereumPay { price: f32, sales_amount: u64 },
}

impl Products {
   
    fn calculate_profit_or_loss(&self) -> f32 {
        match self {
            Products::GameNario { price, sales_amount } => {
                price * (*sales_amount as f32) - 300_000.0
            }
            Products::GameMounterStrike { price, sales_amount } => {
                price * (*sales_amount as f32) - 634_000.0
            }
            Products::FinTechBethereumPay { price, sales_amount } => {
                price * (*sales_amount as f32) - 324_000.0
            }
        }
    }
}
