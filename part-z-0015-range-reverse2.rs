fn main() {

    let nums = (1..18).rev().step_by(3);

    println!("{:?}", nums);

    for num in nums.clone()  {
        
        print!("{:?} ", num);
    }
}

// StepBy { iter: Rev { iter: 1..18 }, step_minus_one: 2, first_take: true }
// 17 14 11 8 5 2
