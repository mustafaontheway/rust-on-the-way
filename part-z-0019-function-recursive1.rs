fn main() {

    let f_5 = factorial(5);

    println!("{f_5}")
}

fn factorial(num: u8) -> u128 {

    if num == 0 {

        1
      
    } else {
        
        num as u128 * factorial(num - 1)
    }
}

