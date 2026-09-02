fn main() {

    let ages: [u8; 4] = [46, 36, 26, 16];

    let her_age = &ages[1]; // reference

    //let her_birth = 2026 - her_age as u16; // error!

    let her_age_as_number = *her_age; // dereferencing -> get value

    let her_birth = 2026 - her_age_as_number as u16;

    println!("Her birth year: {her_birth}") // Her birth year: 1990
}



