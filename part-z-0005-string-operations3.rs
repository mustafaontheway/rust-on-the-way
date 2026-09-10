fn main() {

    let name_1 = "Mustafa".to_string();

    let name_2 = "Kültigin".to_string();

    let last_name = "Büyükdereli".to_string();

    let full_name = format!("{2} {0} {1}", name_1, name_2, last_name);

    println!("Full name: {full_name}")
}

// Full name: Büyükdereli Mustafa Kültigin
