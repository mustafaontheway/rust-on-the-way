fn main() {

    let families = [
        "Mustafa, Aygün, VM, AÖ, P",
        "Volkan, Pelin, Oğuz, Keriman",
        "Rükneddin, Latife, Ayla, Sabri",
    ];

    let people: Vec<_> = families
        .iter()
        .flat_map(|f| f.split(", "))
        .collect();

    println!("{people:?}"); 

    let mut people: Vec<&str> = Vec::new();

    for family in &families {

        for person in family.split(", ") {

            people.push(person);
        }
    }

    println!("{people:?}");
}

// ["Mustafa", "Aygün", "VM", "AÖ", "P", "Volkan", "Pelin", "Oğuz", "Keriman", "Rükneddin", "Latife", "Ayla", "Sabri"]
// ["Mustafa", "Aygün", "VM", "AÖ", "P", "Volkan", "Pelin", "Oğuz", "Keriman", "Rükneddin", "Latife", "Ayla", "Sabri"]
