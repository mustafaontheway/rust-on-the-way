fn main() {

    let mut ages: Vec<u8> = vec![99, 19];

    ages.push(17);
    ages.push(27);
    ages.push(37);
    ages.push(47);

    // for age in ages {

    //     println!("Age: {age}")
    // }

    // println!("Ages: {:?}", ages); // error[E0382]: borrow of moved value: `ages` 

    for age in &ages {

        println!("Age: {age}")
    }

    println!("Ages: {:?}", ages); 

}

// Age: 99
// Age: 19
// Age: 17
// Age: 27
// Age: 37
// Age: 47

// Ages: [99, 19, 17, 27, 37, 47]
