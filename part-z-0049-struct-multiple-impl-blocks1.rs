fn main() {

    let mut player_mustafa = Player::new("mb006587", 92.21, ["Counter Strike", "Super Mario", "Go"]);

    player_mustafa.update_avg_score(4.34);

    player_mustafa.print_player_info();
}

#[derive(Debug)]
struct Player {

    player_id: &'static str,
    player_avg_score: f32,
    player_best_three_games: [&'static str; 3]
}

impl Player {

    fn new(player_id: &'static str, player_avg_score: f32, player_best_three_games: [&'static str; 3]) -> Self {

        Player { player_id, player_avg_score, player_best_three_games }
    }
}

impl Player {
    
    fn update_avg_score(&mut self, s: f32) {

        self.player_avg_score += s;
    }

    fn print_player_info(&self) {

        println!("Player ID: {} and player average score: {}", self.player_id, self.player_avg_score)
    }
}

// Player ID: mb006587 and player average score: 96.55
