fn main() {

    let mut ages: Vec<u8> = Vec::new();

    ages.push(17);
    ages.push(27);
    ages.push(37);
    ages.push(47);

    println!("Ages: {:?}", ages);

    ages.remove(2); // index 2

    println!("Ages: {:?}", ages);

    ages.pop();

    println!("Ages: {:?}", ages);
}

// Ages: [17, 27, 37, 47]
// Ages: [17, 27, 47]
// Ages: [17, 27]
