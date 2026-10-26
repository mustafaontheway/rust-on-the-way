fn main() {

    let ages = vec![33u8, 21, 44, 66, 97, 86, 3];

    let last_even_age = ages.iter().rfind(|age| *age % 2 == 0);

    println!("Last even age: {:?}", last_even_age.unwrap_or(&0));
}

// Last even age: 86
