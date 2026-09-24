fn main() {

    let player_ayhan = Player { player_id: "ab004296".to_string(), player_score: 67.98f32 };

    let player_ayben = Player { player_id: "ank9874".to_string(), player_score: 88u8 };
}

struct Player<T> {

    player_id: String,
    player_score: T
}
