use std::collections::HashMap;

fn main() {

    let mut scores: HashMap<&str, f32> = HashMap::new();

    scores.insert("ayhan bilir", 23.32);

    scores.entry("bengü burada").or_insert(0.0);

    scores.entry("ayhan bilir").or_insert(0.0); // key "ayhan bilir" exists!

    println!("Scores map: {:?}", scores);

    let score_ahyan: &mut f32 = scores.get_mut("ayhan bilir").expect("Not exist");

    *score_ahyan += 11.32;

    println!("Scores map: {:?}", scores);
}

// Scores map: {"ayhan bilir": 23.32, "bengü burada": 0.0}
// Scores map: {"ayhan bilir": 34.64, "bengü burada": 0.0}
