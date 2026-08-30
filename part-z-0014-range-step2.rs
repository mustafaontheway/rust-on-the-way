fn main() {

    let nums = (1..18).step_by(3);

    println!("{:?}", nums);

    // for num in nums  {
        
    //     println!("{:?} ", num);
    // }

    // println!("{:?}", nums); // error[E0382]: borrow of moved value: `nums` 

    for num in nums.clone()  {
        
        println!("{:?} ", num);
    }

    println!("{:?}", nums);
}

