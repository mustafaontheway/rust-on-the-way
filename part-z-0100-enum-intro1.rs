fn main() {

    let my_fav_season = Seasons::Autumn;

    println!("{my_fav_season:?}")
}

#[derive(Debug)]
enum Seasons {

    Spring,
    Summer,
    Autumn,
    Winter
}
