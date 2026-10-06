fn main() {

    let age = "99";

    let age_num= age.parse::<u8>().unwrap();

    println!("Age data: {:?}", age_num);

    let age = "99 years old";

    let age_num= age.parse::<i8>().unwrap_or(-1);

    println!("Age data: {:?}", age_num);
}

// Age data: 99
// Age data: -1
