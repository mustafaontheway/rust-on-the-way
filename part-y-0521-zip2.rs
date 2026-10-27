fn main() {
  
    let first_names = ["Aydın", "Bengü", "Birgün"];

    let last_names = ["Uçar", "Bilir", "Doğar"];

    let full_names_zip = first_names.iter().zip(last_names);

    for (f, l) in full_names_zip {

        println!("{f} {l}")
    }
}

// Aydın Uçar
// Bengü Bilir
// Birgün Doğar
