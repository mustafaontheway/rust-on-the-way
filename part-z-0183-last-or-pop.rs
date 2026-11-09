fn main() {

    let mut ages = vec![22u8, 30, 1, 99, 77];

    let last_age = ages.get(ages.len() - 1);

    println!("{last_age:?}");

    println!("Ages: {ages:?}");

    println!("----------------------------");

    let last_age = ages.last();

    println!("{last_age:?}");

    println!("Ages: {ages:?}");

    println!("----------------------------");

    let last_age = ages.pop();

    println!("{last_age:?}");

    println!("Ages: {ages:?}");
}

// Some(77)
// Ages: [22, 30, 1, 99, 77]
// ----------------------------
// Some(77)
// Ages: [22, 30, 1, 99, 77]
// ----------------------------
// Some(77)
// Ages: [22, 30, 1, 99]
