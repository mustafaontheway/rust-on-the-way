fn main() {

    let ages = [17u8, 99, 21, 87, 65, 40, 33];

    let mut ages_slice = ages[..=3].to_vec();

    ages_slice[1] = 109;

    println!("Ages slice: {ages_slice:?}");

    println!("Ages: {:?}", &ages);
}

// Ages slice: [17, 109, 21, 87]
// Ages: [17, 99, 21, 87, 65, 40, 33]
