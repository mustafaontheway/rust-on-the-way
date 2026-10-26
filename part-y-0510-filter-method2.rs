fn main() {

    let ages = vec![33u8, 21, 44, 66, 97, 86, 3];

    // let odd_ages: Vec<&u8> = ages.iter().filter(|age| *age % 2 != 0).collect();

    // println!("{:?}", odd_ages);

    // let even_ages: Vec<u8> = ages.iter().filter(|age| *age % 2 == 0).copied().collect();

    // println!("{:?}", even_ages);

    //let's use for loop...

    let mut odd_ages: Vec<u8> = Vec::new();

    let mut even_ages: Vec<u8> = Vec::new();

    for age in &ages {

        if age % 2 == 0 {

            even_ages.push(*age);
        }

        else {

            odd_ages.push(*age);
        }
    }

    println!("Odd ages: {:?}", odd_ages);

    println!("Even ages: {:?}", even_ages);
}

// Odd ages: [33, 21, 97, 3]
// Even ages: [44, 66, 86]
