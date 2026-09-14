use std::collections::HashMap;

fn main() {

    let family_ages: [(&str, u8); 3] = [("Ayhan", 29), ("Beyhan", 99), ("Bengü", 45)];

    let _map_family_ages = HashMap::from(family_ages);
}

