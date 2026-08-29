fn main() {

    let _ages: [u8; 4] = [12, 99, 1, 77];

    let mut rates: [f32; 3] = [11.33, 22.45, 7.566];

    rates[2] = 22.5447;

    println!("Rates: {rates:?}") 
}

// Rates: [11.33, 22.45, 22.5447]
