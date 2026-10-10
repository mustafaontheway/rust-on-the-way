fn main() {

    let year: u16 = 2026;

    let this_year = year; // copy trait

    println!("{year} is {this_year}.");

    {
        println!("This year is {year}.");

        let _her_birth_year: u16 = 1990; // scope
    }

    //println!("Her birth year is {_her_birth_year}") // lifetime & scope error
}

// 2026 is 2026.
// This year is 2026.
