fn main() {
 
    let citizen_ages = vec![17u8, 85, 24, 10, 8, 96, 75, 21];

    let not_adults: Vec<bool> = citizen_ages.iter().map(|age| *age < 18).collect();

    println!("Not adults: {:?}", not_adults) // Not adults: [true, false, false, true, true, false, false, false]
}

