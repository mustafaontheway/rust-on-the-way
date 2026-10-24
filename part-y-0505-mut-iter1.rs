fn main() {

    let mut ages = vec![22u8, 14, 9, 95, 7];

    let ages_iterator = ages.iter_mut();

    for age in ages_iterator {

        *age -= 2;
    }

    println!("Updated Ages: {:?}", ages); // Updated Ages: [20, 12, 7, 93, 5]

    // for age in &mut ages {

    //     *age -= 2;
    // }

    // println!("Updated Ages: {:?}", ages); // Updated Ages: [20, 12, 7, 93, 5]
}

