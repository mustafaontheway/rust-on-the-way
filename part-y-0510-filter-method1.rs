fn main() {

    let ages = vec![33u8, 21, 44, 66, 97, 86, 3];

    let odd_ages: Vec<&u8> = ages.iter().filter(|age| *age % 2 != 0).collect();

    println!("{:?}", odd_ages);

    let even_ages: Vec<u8> = ages.iter().filter(|age| *age % 2 == 0).copied().collect();

    println!("{:?}", even_ages);
}

// [33, 21, 97, 3]
// [44, 66, 86]
