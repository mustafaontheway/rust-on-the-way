fn main() {

    check_voter(77);

    check_voter(17);
}

fn check_voter(age: u8) {

    let c = if age >= 18 { "He/she can vote" } else { "He/she can't vote" };
        
    println!("{c}")
}

// He/she can vote
// He/she can't vote
