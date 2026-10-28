use rand::random;

fn main() {

    let random_float: f32 = rand::random();

    println!("Random float: {random_float}");

    let random_positive_num: u16 = rand::random();

    println!("Random positive num: {random_positive_num}");

    let random_int: i32 = random();

    println!("Random negative or positive num: {random_int}")
}

// Random float: 0.8196505
// Random positive num: 13547
// Random negative or positive num: -1836396357
