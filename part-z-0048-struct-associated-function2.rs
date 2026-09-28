fn main() {

    let mut _player_mustafa = Player::new("mb006587", 92.21, ["Counter Strike", "Super Mario", "Go"]);
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

