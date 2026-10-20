use std::process;

fn main() {

    let sales: f32 = 340_000.0;

    let cost: f32 = 400_000.0;

    let premium: f32;

    if sales < cost {

        process::exit(AppExitCodes::SalesBelowCost as i32)

    } else {

        premium = (sales - cost) * 0.04;
    }

    println!("Premium amount: {premium} ₺.");
}

enum AppExitCodes {

    Success = 0,
    GeneralError = 1,
    SalesBelowCost = 2,
}

// error: process didn't exit successfully: `target\debug\hi.exe` (exit code: 2)
