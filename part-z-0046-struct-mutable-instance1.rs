fn main() {

    let mut player1 = Player { player_id: "au004296".to_string(), player_score: 65.32 };

    player1.update_score(-3.45);

    println!("{:?}", player1)

}

#[derive(Debug)]
struct Player {

    player_id: String,
    player_score: f32
}

impl Player {

    fn update_score(&mut self, s: f32) {

        self.player_score += s
    }
}

// Player { player_id: "au004296", player_score: 61.87 }
