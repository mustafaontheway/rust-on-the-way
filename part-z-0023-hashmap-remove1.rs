use std::collections::HashMap;

fn main() {

    let family_ages: [(&str, u8); 3] = [("Ayhan", 29), ("Beyhan", 99), ("Bengü", 45)];

    let mut map_family_ages = HashMap::from(family_ages);

    let dead_member = map_family_ages.remove("Beyhan");

    println!("{}", dead_member.unwrap());

    println!("{:?}", dead_member);
}

// 99
// Some(99)
