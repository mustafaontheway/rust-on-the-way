fn main() {

    let mut ages: Vec<u8> = vec![99, 19];

    ages.push(17);
    ages.push(27);
    ages.push(37);
    ages.push(47);

    println!("Ages: {:?}", ages.get(2)); 

    println!("Ages: {:?}", ages[2]); 

    println!("Ages: {:?}", ages.get(22)); // index 22?

    println!("Ages: {:?}", ages[22]); // Error: index out of bounds: the len is 6 but the index is 22

}

// Ages: Some(17)
// Ages: 17
// Ages: None
