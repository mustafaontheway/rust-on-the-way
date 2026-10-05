fn main() {
  
    let ages: [u8; 7] = [22, 99, 65, 24, 7, 21, 100];

    let (age, age_exist)= get_age(&ages, 3);

    println!("{:?}", age);

    println!("{:?}", age_exist);

    let (age, age_exist) = get_age(&ages, 23);

    println!("{:?}", age);

    println!("{:?}", age_exist);
}

fn get_age(arr: &[u8], index_val: usize) -> (u8, bool) {

    let info = match arr.get(index_val) {
        Some(age) => (*age, true),
        None => (0, false),
    };

    if info.1 == false {
        println!("The age you're looking for not exist!")
    }
    
    (info.0, info.1)
}

// 24
// true
// The age you're looking for not exist!
// 0
// false
