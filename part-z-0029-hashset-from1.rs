use std::collections::HashSet;

fn main() {

    let members = ["Ayhan Bilir", "Ayhan Bilir", "Ayhan Bilir", "Bengü Burada"];

    let unique_members = HashSet::from(members);

    println!("Unique Members: {:?}", unique_members)
}

// Unique Members: {"Ayhan Bilir", "Bengü Burada"}
