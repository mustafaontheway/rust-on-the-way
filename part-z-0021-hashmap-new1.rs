use std::collections::HashMap;

fn main() {

    let mut famliy_ages: HashMap<&'static str, u8> = HashMap::new();

    famliy_ages.insert("Ayhan", 29);

    famliy_ages.insert("Aybüke", 37);

    println!("{:?}", famliy_ages)
}

// {"Aybüke": 37, "Ayhan": 29}
