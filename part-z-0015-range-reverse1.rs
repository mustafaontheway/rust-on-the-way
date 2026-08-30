fn main() {

    let nums = (1..18).rev();

    println!("{:?}", nums);

    for num in nums.clone()  {
        
        print!("{:?} ", num);
    }
}

// Rev { iter: 1..18 }
// 17 16 15 14 13 12 11 10 9 8 7 6 5 4 3 2 1 
