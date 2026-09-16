fn main() {

    let mut ages: Vec<u8> = vec![10, 20, 30, 40];

    let our_ages = [77u8, 99, 109, 89];

    ages.splice(2.., our_ages);

    println!("Ages: {:?}", ages);
}

// Ages: [10, 20, 77, 99, 109, 89]
