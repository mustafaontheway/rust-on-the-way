fn main() {

    let p = calculate_premium(Ok(2_300_000));

    println!("{p}");

    let p = calculate_premium(Ok(0));

    println!("{p}");

    let p = calculate_premium(Err("We don't have sales data..."));

     println!("{p}");
}

fn calculate_premium(sales: Result<u64, &str>) -> f64 {

    match sales {
        
        Ok(s) if s > 1_000_000 => s as f64 * 0.07,
        Ok(s) if s > 500_000 => s as f64 * 0.03,
        Ok(_) => 0.0 ,
        Err(err) => {
            println!("Error: {err}");
            0.0
        }
    }
}

