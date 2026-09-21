fn main() {

    let full_name = "Mustafa Büyükdereli".to_string();

    let first_name = &full_name[..=6];

    println!("{first_name}");

    let last_name = &full_name[8..];

    println!("{last_name}");
}

// Mustafa
// Büyükdereli
