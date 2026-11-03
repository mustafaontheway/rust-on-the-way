fn main() {

    let player_aykan_time = PlayTime::Minutes(47);

    let player_aysel_time = PlayTime::Minutes(23);

    println!("Playing times equal? => {:?}", player_aykan_time == player_aysel_time) // Playing times equal? => false
}

#[derive(Debug, PartialEq)]
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



