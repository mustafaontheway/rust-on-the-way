fn main() {

    let ages = vec![77u8, 21, 16, 95, 33];

    let ages_iterator = ages.into_iter();

    for age in ages_iterator {

        println!("{age}")
    }
}

