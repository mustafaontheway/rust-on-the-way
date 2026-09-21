fn main() {

    let full_name = "Mustafa Büyükdereli".to_string();

    let first_name = &full_name[..=6];

    let last_name = &full_name[8..];

    let user_middle_name = String::from("Bilge");

    print_info(first_name);
    print_info(last_name);

    print_info(&user_middle_name);

}

fn print_info(info: &str) {

    println!("User info: {info}")
}
