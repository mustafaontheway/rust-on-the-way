fn main() {

    let ages = vec![33u8, 21, 44, 66, 97, 86, 3];

    let first_even_age = ages.iter().find(|age| *age % 2 == 0);

    println!("First even age: {:?}", first_even_age.unwrap_or(&0));
}

// First even age: 44
