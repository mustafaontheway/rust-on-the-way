fn main() {

    let ages = vec![77u8, 21, 16, 95, 33];

    let mut ages_iterator = ages.into_iter();

    println!("{ages_iterator:?}"); // IntoIter([77, 21, 16, 95, 33])

    println!("Age: {:?}", ages_iterator.next()); 

    println!("{ages_iterator:?}");

    println!("Age: {:?}", ages_iterator.next()); 

    println!("{ages_iterator:?}");

    println!("Age: {:?}", ages_iterator.next()); 

    println!("{ages_iterator:?}");

    println!("Age: {:?}", ages_iterator.next()); 

    println!("{ages_iterator:?}");

    println!("Age: {:?}", ages_iterator.next()); 

    println!("{ages_iterator:?}");
}

// IntoIter([77, 21, 16, 95, 33])
// Age: Some(77)
// IntoIter([21, 16, 95, 33])
// Age: Some(21)
// IntoIter([16, 95, 33])
// Age: Some(16)
// IntoIter([95, 33])
// Age: Some(95)
// IntoIter([33])
// Age: Some(33)
// IntoIter([])
