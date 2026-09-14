use std::collections::HashSet;

fn main() {

    let mut members = HashSet::new();

    members.insert("Ayhan");
    members.insert("Bengü");
    members.insert("Ayhan");
    members.insert("Ayhan");
    members.insert("Ayhan");

    println!("Members: {:?}", members)
}

// Members: {"Ayhan", "Bengü"}
