fn main() {

    let player1 = Players::Player1 { player_name: "Ayhan Bilir".to_string(), player_score: 3400 };

    // let's ignore name!

    let mut score = if let Players::Player1 { player_score, .. } = player1 {
        
        player_score
    } else {
      
        0
    };

    score += 300;

    println!("Updated score: {score}") // Updated score: 3700
}

#[derive(Debug)]
enum Players {

    Player1 { player_name: String, player_score: u16},
    Player2 { player_name: String, player_score: u16},
    Player3 { player_name: String, player_score: u16},
}
