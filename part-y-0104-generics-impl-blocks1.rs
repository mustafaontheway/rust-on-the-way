fn main() {

    let mut player1 = Player::new("au004296", 76.43);

    player1.update_power(-3.21);

    println!("{:?}", player1);

    let mut player2: Player<u8> = Player::new("mb006587", 89);

    player2.update_power(5);

    println!("Player 2 new power: {:?}", player2.power);
}

use std::ops::AddAssign;

#[derive(Debug)]
struct Player<T> {

    player_id: &'static str,
    power: T
}

impl<T> Player<T>
where 
    T: AddAssign
{
    
    fn new( player_id: &'static str, power: T) -> Self {

        Player { player_id, power }
    }

    fn update_power(&mut self, p: T) {

        self.power += p
    } 
}

// Player { player_id: "au004296", power: 73.22000000000001 }
// Player 2 new power: 94
