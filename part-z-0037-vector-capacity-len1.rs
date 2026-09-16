fn main() {

    let mut ages: Vec<u8> = vec![99, 19];

    println!("Ages capacity: {}", ages.capacity());

    ages.push(17);
    ages.push(27);
    ages.push(37);

    println!("Ages capacity: {}", ages.capacity());

    println!("Ages len: {}", ages.len());

    ages.push(47);
}

// Ages capacity: 2
// Ages capacity: 8
// Ages len: 5
