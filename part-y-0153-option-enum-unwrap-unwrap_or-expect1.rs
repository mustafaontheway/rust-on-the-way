fn main() {

    let years: Vec<u16> = vec![2000, 2008, 2013, 2016, 2018, 2020, 2026];

    let start_year = years.get(1).unwrap(); 

    println!("Year option unwrap : {start_year:?}");

    let unexpected_year = years.get(21);

    println!("Year option 2: {unexpected_year:?}");

    unexpected_year.expect("Please, consider the vector length!");
}

// Year option unwrap : 2008
// Year option 2: None

// thread 'main' (12872) panicked at src\main.rs:13:21:
// Please, consider the vector length!
