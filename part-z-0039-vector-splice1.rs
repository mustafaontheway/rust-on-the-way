fn main() {

    let mut ages: Vec<u8> = vec![99, 19];

    let our_ages = [77u8, 99, 100, 110];

    ages.splice(ages.len().., our_ages);

    println!("Ages: {:?}", ages);
}

// Ages: [99, 19, 77, 99, 100, 110]
