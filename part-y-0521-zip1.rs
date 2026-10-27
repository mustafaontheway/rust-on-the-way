fn main() {
  
    let first_names = ["Aydın", "Bengü", "Birgün"];

    let last_names = ["Uçar", "Bilir", "Doğar"];

    let full_names = first_names.iter().zip(last_names);

    println!("{full_names:?}"); // Zip

    let full_names: Vec<String> = first_names
        .iter()
        .zip(last_names)
        .map(|(first_name, last_name)| format!("{last_name} {first_name}"))
        .collect();

    println!("{full_names:?}"); // ["Uçar Aydın", "Bilir Bengü", "Doğar Birgün"]
}
