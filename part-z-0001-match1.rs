fn main() {

    let is_completed = true;

    let final_result = match is_completed {

        true => "Awesome",
        false => "Why!"
    };

    println!("{final_result}")
}

// Awesome
