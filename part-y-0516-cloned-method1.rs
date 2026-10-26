fn main() {
    let full_names = vec![
        "Mustafa Büyükdereli",
        "Aykan Bilir",
        "Aygün Büyükdereli",
        "Ayben Burada",
    ];

    let buyukdereli_family: Vec<String> = full_names
        .iter()
        .map(|name| name.to_lowercase())
        .filter(|name| name.contains("büyükdereli"))
        .collect();

    println!("Büyükdereli family: {:?}", buyukdereli_family); 

    let buyukdereli_family: Vec<String> = full_names
        .iter()
        .cloned()
        .map(|name| name.to_lowercase())
        .filter(|name| name.contains("büyükdereli"))
        .collect();

    println!("Büyükdereli family: {:?}", buyukdereli_family);

}

// Büyükdereli family: ["mustafa büyükdereli", "aygün büyükdereli"]
// Büyükdereli family: ["mustafa büyükdereli", "aygün büyükdereli"]
