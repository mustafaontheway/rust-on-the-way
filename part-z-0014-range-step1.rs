fn main() {

    let nums = (1..18).step_by(3);

    println!("{:?}", nums);

    for num in nums  {
        
        println!("{:?} ", num);
    }
}

// StepBy { iter: 1..18, step_minus_one: 2, first_take: true }

// 1 
// 4 
// 7 
// 10 
// 13 
// 16 
