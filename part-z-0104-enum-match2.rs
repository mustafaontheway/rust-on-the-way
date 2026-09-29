fn main() {

    let year_for_canada = since_milesstone_year(G7::Canada, 2000);

    println!("{year_for_canada}"); // 133

    let year_for_italy = since_milesstone_year(G7::Italy, 1700);

    println!("{year_for_italy}"); // -161 -> Logical result! Think! Why?
}

#[derive(Debug)]
enum G7 {

    USA,
    Canada,
    Japan,
    Germany,
    Italy,
    UnitedKingdom,
    France
}

fn since_milesstone_year(country: G7, search_year: i16) -> i16 {

    match country {

        G7::USA => search_year - 1776,
        G7::Canada => search_year - 1867,
        G7::Japan => search_year - 1952,
        G7::Germany => search_year - 1990,
        G7::Italy => search_year - 1861,
        G7::UnitedKingdom => search_year - 1707,
        G7::France => search_year - 1789
    }
}

