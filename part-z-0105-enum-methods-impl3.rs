fn main() {

    let player_ayben_time = PlayTime::Hour(4);

    let player_aykan_time = PlayTime::Minutes(47);

    let player_aysel_time = PlayTime::Minutes(23);

    //let total_play_time = player_ayben_time * 60 + player_ayben_time + player_aysel_time; // Error!

    let total_play_time = player_ayben_time.to_minute() + player_aykan_time.to_minute() + player_aysel_time.to_minute();

    println!("{total_play_time}") // 310
}

enum PlayTime {

    Hour(u32),
    Minutes(u32)
}

impl PlayTime {
    
    fn to_minute(&self) -> u32 {

        match self {
            
            PlayTime::Hour(h) => *h * 60,
            PlayTime::Minutes(m) => *m
        }
    }
}



