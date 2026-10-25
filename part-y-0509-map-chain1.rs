fn main() {

    let names = vec!["Mustafa ", " AYHAN ", "kAğan"];

    let updated_names: Vec<String> = names.iter().map(|n| n.trim()).map(|n| n.to_lowercase()).collect();

    println!("Updated names: {:?}", updated_names)
}

// Updated names: ["mustafa", "ayhan", "kağan"]
