fn main() {

    let canada_my = find_milesstone_year(G7::Canada);

    println!("Canada milesstone year: {canada_my}")
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

fn find_milesstone_year(country: G7) -> u16 {

    match country {

        G7::USA => 1776,
        G7::Canada => 1867,
        G7::Japan => 1952,
        G7::Germany => 1990,
        G7::Italy => 1861,
        G7::UnitedKingdom => 1707,
        G7::France => 1789
    }
}

//Canada milesstone year: 1867
