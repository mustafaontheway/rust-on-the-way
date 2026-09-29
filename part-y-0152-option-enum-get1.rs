fn main() {

    let years: Vec<u16> = vec![2000, 2008, 2013, 2016, 2018, 2020, 2026];

    let start_year = years.get(1); 

    println!("Year option 1: {start_year:?}");

    let unexpected_year = years.get(21); 

    println!("Year option 2: {unexpected_year:?}");

    // let unexpected_year_without_option = years[21]; 

    // println!("Year without option: {unexpected_year_without_option:?}"); // Error: "index out of bounds: the len is 7 but the index is 21"
}

// Year option 1: Some(2008)
// Year option 2: None
