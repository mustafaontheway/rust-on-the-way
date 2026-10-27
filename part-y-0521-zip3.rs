fn main() {
  
    let first_names = ["Aydın", "Bengü", "Birgün"];

    let last_names = ["Uçar", "Bilir", "Doğar", "Kağan"];

    let full_names_zip = first_names.iter().zip(last_names);

    for (f, l) in full_names_zip {

        if first_names.len() == last_names.len() {

            println!("{f} {l}")
        }
    }
    println!("The length of the lists are NOT equal!")
}

// The length of the lists are NOT equal!
