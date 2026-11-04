use std::collections::HashMap;

fn main() {

    let mut scores: HashMap<&str, f32> = HashMap::new();

    scores.insert("ayhan bilir", 23.32);

    scores.entry("bengü burada").or_insert(0.0);

    scores.entry("ayhan bilir").or_insert(0.0); // key "ayhan bilir" exists!

    println!("Scores map: {:?}", scores); // Scores map: {"bengü burada": 0.0, "ayhan bilir": 23.32}  
}

