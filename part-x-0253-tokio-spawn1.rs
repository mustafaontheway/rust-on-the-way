//use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    let task_sum3 = tokio::spawn(sum3());

    let task_sum2 = tokio::spawn(sum2());

    let task_sum1 = tokio::spawn(sum1());
    
    let _ = tokio::join!(task_sum1, task_sum2, task_sum3);
}

async fn sum1() -> u64 {

    let mut total = 0u64;

    for i in 0..500 {

        total += i;
    }
    println!("Sum 1 function ended!");

    total
}

async fn sum2() -> u64 {

    let mut total = 0u64;

    for i in 0..50000 {

        total += i;
    }
    println!("Sum 2 function ended!");

    total
}

async fn sum3() -> u64 {

    let mut total = 0u64;

    for i in 0..500000 {

        total += i;
    }
    println!("Sum 3 function ended!");

    total
}

// Sum 1 function ended!
// Sum 2 function ended!
// Sum 3 function ended!
