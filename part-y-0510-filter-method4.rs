fn main() {
  
    let full_names = vec![
        "Mustafa Büyükdereli",
        "Aykan Bilir",
        "Aygün Büyükdereli",
        "Ayben Burada",
    ];

    let buyukdereli_family: Vec<&str> = full_names
        .iter()
        .filter(|name| name.to_lowercase().contains("büyükdereli"))
        .copied()
        .collect();

    println!("Büyükdereli family: {:?}", buyukdereli_family); 
}

// Büyükdereli family: ["Mustafa Büyükdereli", "Aygün Büyükdereli"]
