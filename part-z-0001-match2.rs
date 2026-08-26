fn main() {

    let month: u8 = 12;

    match month {
        3 | 4 | 5 => println!("Spring"),
        6..=8 => println!("Summer"),
        9..12 => println!("Autumn"),
        12 | 1 | 2 => println!("Winter"),
        _ => println!("12 months!")
    }
}

// Winter
