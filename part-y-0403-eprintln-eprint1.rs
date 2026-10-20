fn main() {

    let sales: f32 = 340_000.0;

    let cost: f32 = 400_000.0;

    let premium: f32;

    if sales < cost {

        eprintln!("Low sales amount...");

        premium = 0.0;

    } else {

        premium = (sales - cost) * 0.04;
    }

    println!("Premium amount: {premium} ₺.");
}

// cargo run > output.txt
