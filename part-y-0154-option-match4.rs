fn main() {

    let p = calculate_premium(Some(2_300_000));

    println!("{p}");

    let p = calculate_premium(Some(300_000));

    println!("{p}");
}

fn calculate_premium(sales: Option<u64>) -> f64 {

    match sales {
        
        Some(s) if s > 1_000_000 => s as f64 * 0.07,
        Some(s) if s > 500_000 => s as f64 * 0.03,
        Some(_) | None => 0.0 
    }
}

