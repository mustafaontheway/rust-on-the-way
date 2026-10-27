fn main() {

    let member_ages = vec![22u8, 57, 96, 45, 14, 5, 55];

    let (p1, p2): (Vec<u8>, Vec<u8>) = member_ages
        .iter()
        .copied()
        .partition(|age| *age > 20 && *age < 60);

    println!("{p1:?}");
    println!("{p2:?}");

    println!("--------------------");

    // without iterator

    let mut p1: Vec<u8> = Vec::new();
    let mut p2: Vec<u8> = Vec::new();

    for age in &member_ages {

        if *age > 20 && *age < 60 {

            p1.push(*age);
        } 
        else {

            p2.push(*age);
        }
    }

    println!("{p1:?}");
    println!("{p2:?}");
}

// [22, 57, 45, 55]
// [96, 14, 5]
// --------------------
// [22, 57, 45, 55]
// [96, 14, 5]
