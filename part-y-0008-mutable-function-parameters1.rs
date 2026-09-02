fn main() {

    update_person_info("Mustafa".to_string(), 99);

    update_person_info("Aybüke".to_string(), 17);
}

fn update_person_info(mut name: String, age: u8) {

    if age >= 18 {

        name.push_str(" is adult.");

        println!("{name}")

    } else {

        name.push_str(" is child.");

        println!("{name}")
    }
}

// Mustafa is adult.
// Aybüke is child.
