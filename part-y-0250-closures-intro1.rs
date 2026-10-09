fn main() {

    let greet = |name: &str| println!("Hi {name}!"); 

    greet("Mustafa");

    let sum_nums = |x: i128, y: i128| -> i128 { x + y };

    println!("{}", sum_nums(500, -655_222))
}

// Hi Mustafa!
// -654722
