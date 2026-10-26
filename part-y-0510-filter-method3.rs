fn main() {
    let names = vec!["Mustafa", "Ayhan", "AYBEN", "KAĞAN", "Bilge"];

    let contains_turkish_char: Vec<String> = names
        .iter()
        .map(|name| name.to_lowercase())
        .filter(|name| name.contains('ğ'))
        .collect();

    println!("{:?}", contains_turkish_char) // ["kağan"]
}
