fn main() {

    let nums = 1..8;

    println!("{:?}", nums);

    for num in nums  {
        
        print!("{:?} ", num);
    }

    println!("\n..........................");

    let nums = 1..=8;

    println!("{:?}", nums);

    for num in nums  {
        
        print!("{:?} ", num);
    }
}

// 1..8
// 1 2 3 4 5 6 7 
// ..........................
// 1..=8
// 1 2 3 4 5 6 7 8 
