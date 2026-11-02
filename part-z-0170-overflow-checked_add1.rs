fn main() {

    let salary: u16 = 60_000;

    let new_salary = salary.checked_add(2_000);

    println!("New salary: {new_salary:?}");

    let new_salary = salary.checked_add(12_000);

    println!("New salary: {new_salary:?}")
}

// New salary: Some(62000)
// New salary: None
