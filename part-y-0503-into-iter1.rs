fn main() {

    let ages = vec![77u8, 21, 16, 95, 33];

    let ages_iterator = ages.into_iter();

    println!("{ages_iterator:?}"); // IntoIter([77, 21, 16, 95, 33])

    //println!("{ages:?}"); // Error => borrow of moved value: `ages`
}

