use std::process;

fn main() {

    let sales: f32 = 340_000.0;

    let cost: f32 = 400_000.0;

    let premium: f32;

    if sales < cost {

        process::exit(1)

    } else {

        premium = (sales - cost) * 0.04;
    }

    println!("Premium amount: {premium} ₺.");
}

// error: process didn't exit successfully: `target\debug\hi.exe` (exit code: 1)


