fn main() {
    let ages: [u8; 7] = [22, 99, 65, 24, 7, 21, 100];

    let age1 = ages.get(22).copied().unwrap_or(0);

    println!("{age1}") // 0
}


