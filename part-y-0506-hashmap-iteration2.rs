use std::collections::HashMap;

fn main() {

    let char_of_my_name_map = count_chars("Mustafa Büyükdereli");

    println!("{char_of_my_name_map:?}");

    let char_of_my_name_map = count_chars(&"Mustafa Büyükdereli".to_lowercase());

    println!("{char_of_my_name_map:?}")
}

fn count_chars(txt: &str) -> HashMap<char, u8> {

    let mut char_map = HashMap::new();

    for word in txt.split_whitespace() {

        for ch in word.chars() {

            *char_map.entry(ch).or_insert(0) += 1;
        }
    }

    char_map
}

// {'k': 1, 'B': 1, 's': 1, 'r': 1, 'd': 1, 'f': 1, 'a': 2, 'M': 1, 'e': 2, 'ü': 2, 't': 1, 'u': 1, 'l': 1, 'y': 1, 'i': 1}
// {'s': 1, 'e': 2, 'u': 1, 'a': 2, 'k': 1, 'l': 1, 'b': 1, 't': 1, 'r': 1, 'i': 1, 'ü': 2, 'f': 1, 'm': 1, 'd': 1, 'y': 1}
