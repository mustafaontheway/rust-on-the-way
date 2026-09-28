fn main() {

    let mut player_aykan = Player::new("au004296", 75.34);

    player_aykan.update_score(-12.21);

    player_aykan.print_player_info();
}

#[derive(Debug)]
struct Player {

    player_id: &'static str,
    player_score: f32
}

impl Player {

    fn new(player_id: &'static str, player_score: f32) -> Self {

        Player { player_id, player_score }
    }

    fn update_score(&mut self, s: f32) {

        self.player_score += s
    }

    fn print_player_info(&self) {

        println!("Player ID: {} and player score: {}", self.player_id, self.player_score);
    }
}

//Player ID: au004296 and player score: 63.129997
