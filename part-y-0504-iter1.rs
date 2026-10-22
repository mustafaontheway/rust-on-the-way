fn main() {

    let ages = vec![77u8, 21, 16, 95, 33];

    let mut ages_iterator = ages.iter();

    println!("Age: {:?}", ages_iterator.next());

    println!("Age: {:?}", ages_iterator.next());

    println!("Ages: {:?}", ages);

    let youngest = ages_iterator.next();

    println!("Youngest age: {:?}", youngest.copied().unwrap_or(0));
}

// Age: Some(77)
// Age: Some(21)
// Ages: [77, 21, 16, 95, 33]
// Youngest age: 16
