fn main() {

    let pd1 = update_person_info("Mustafa".to_string(), 99);

    println!("{}", &pd1);

    let pd2 = update_person_info("Aybüke".to_string(), 17);

    println!("{}", &pd2);
}

fn update_person_info(mut name: String, age: u8) -> String {

    if age >= 18 {

        name.push_str(" is adult.");

        name

    } else {

        name.push_str(" is child.");

        name
    }
}

// Mustafa is adult.
// Aybüke is child.
