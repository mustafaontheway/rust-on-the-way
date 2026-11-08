use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    println!("Tokio started!");

    let mut counter: u64 = 0;

    for i in 0..5 {

        counter += i;

        sleep(Duration::from_secs(counter)).await;
        
        println!("Counter: {counter}")
    }

    println!("Tokio ended!")
}

// Tokio started!
// Counter: 0
// Counter: 1
// Counter: 3
// Counter: 6
// Counter: 10
// Tokio ended!
