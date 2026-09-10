fn main() {

    let name_1 = "Mustafa".to_string();

    let name_2 = "Kültigin".to_string();

    let last_name = "Büyükdereli".to_string();

    // let full_name = name_1 + " " + &name_2 + " " + last_name; // Error: main.rs(9, 52): consider borrowing here: `&`

    let full_name = name_1 + " " + &name_2 + " " + &last_name; 

    println!("Full name: {full_name}")
}

// Full name: Mustafa Kültigin Büyükdereli
