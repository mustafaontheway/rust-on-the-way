fn main() {

    let ages = vec![22u8, 14, 99, 75, 18, 23];

    // without the collect method!
    // i want to show what happens behind the scene

    let mut updated_ages: Vec<u8> = Vec::new();

    for age in &ages{

        updated_ages.push(*age - 2);
    }

    println!("{ages:?}");

    println!("{updated_ages:?}");
}

// [22, 14, 99, 75, 18, 23]
// [20, 12, 97, 73, 16, 21]
