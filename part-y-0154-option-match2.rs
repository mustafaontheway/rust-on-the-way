fn main() {
    let ages: [u8; 7] = [22, 99, 65, 24, 7, 21, 100];

    let age1 = get_age(&ages, 3);

    println!("{:?}", age1);

    let age2 = get_age(&ages, 23);

    println!("{:?}", age2);
}

fn get_age(arr: &[u8], index_val: usize) -> u8 {

    let age = match arr.get(index_val) {
        Some(age) => *age,
        None => 0,
    };

    if age == 0 {
        println!("The age you're looking for not exist!")
    }
    
    age
}

// 24
// The age you're looking for not exist!
// 0
