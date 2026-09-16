fn main() {

    let mut ages: Vec<u8> = vec![99, 19];

    ages.push(17);
    ages.push(27);
    ages.push(37);
    ages.push(47);

    println!("Ages slice 1: {:?}", &ages[2..]); 

    println!("Ages slice 2: {:?}", &ages[..3]);

    println!("Ages slice 3: {:?}", &ages[..=3]);

    let age_slice = &ages[1..=4];

    println!("Ages slice 4: {:?}", age_slice)
}

// Ages slice 1: [17, 27, 37, 47]
// Ages slice 2: [99, 19, 17]
// Ages slice 3: [99, 19, 17, 27]
// Ages slice 4: [19, 17, 27, 37]
