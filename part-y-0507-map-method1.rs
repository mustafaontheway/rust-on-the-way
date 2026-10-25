fn main() {

    let ages = vec![22u8, 14, 99, 75, 18, 23];

    let mut ages_iterator = ages.into_iter().map(|age| age - 2);

    println!("{:?}", ages_iterator);

    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));
    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));
    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));
    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));
    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));
    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));

    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));
    println!("Updated age: {:?}", ages_iterator.next().unwrap_or(0));

    //println!("Ages: {:?}", ages) // borrow of moved value: `ages`
}

// Map { iter: IntoIter([22, 14, 99, 75, 18, 23]) }
// Updated age: 20
// Updated age: 12
// Updated age: 97
// Updated age: 73
// Updated age: 16
// Updated age: 21
// Updated age: 0
// Updated age: 0

    
