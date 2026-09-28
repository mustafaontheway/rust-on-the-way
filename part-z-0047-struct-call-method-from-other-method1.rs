fn main() {

    let mut player1 = Player { player_id: "au004296", player_score: 65.32 };

    player1.update_score(-3.45);

    player1.print_score();

    player1.print_player_info();
}

#[derive(Debug)]
struct Player {

    player_id: &'static str,
    player_score: f32
}

impl Player {

    fn update_score(&mut self, s: f32) {

        self.player_score += s
    }

    fn print_score(&self) {

        println!("Player score: {}", self.player_score)
    }

    fn print_player_info(&self) {

        println!("Player ID: {}", self.player_id);

        self.print_score();
    }
}

// Player score: 61.87
// Player ID: au004296
// Player score: 61.87
