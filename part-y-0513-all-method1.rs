fn main() {
  
    let some_words = vec!["Yasa", "Er", "Erkek", "AĞABEY", "Bilge", "Ağaç"];

    let words_contain_turkish_char: Vec<String> = some_words
        .iter()
        .map(|word| word.to_lowercase())
        .filter(|word| word.contains('ğ'))
        .collect();

    println!("{:?}", words_contain_turkish_char); // ["ağabey", "ağaç"]

    let words_contain_turkish_char = some_words.iter().all(|word| word.contains('Ğ') || word.contains('ğ'));

    println!("{:?}", words_contain_turkish_char); // false => not all!
}
