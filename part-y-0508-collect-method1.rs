fn main() {

    let ages = vec![22u8, 14, 99, 75, 18, 23];

    let updated_ages: Vec<u8> = ages.iter().map(|age| age - 2).collect();

    println!("Ages: {:?}", ages);

    println!("Updated ages: {:?}", updated_ages);
}

// Ages: [22, 14, 99, 75, 18, 23]
// Updated ages: [20, 12, 97, 73, 16, 21]
