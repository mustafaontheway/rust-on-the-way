fn main() {

    let player_mustafa_score = Scores::Player1(3700);

    let numeric_player_mustafa_score: u16 = if let Scores::Player1(s) = player_mustafa_score { s } else { 0 };

    println!("{numeric_player_mustafa_score}")    
}

#[derive(Debug)]
enum Scores {

    Player1(u16),
    Player2(u16),
    Player3(u16),
}
