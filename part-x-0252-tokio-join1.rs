use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {

    tokio::join!(
        completed_1(),
        completed_2(),
        completed_3()
    );
}

async fn completed_1() -> &'static str {

    for _i in 0..500 {

        sleep(Duration::from_millis(10)).await;
    }

    println!("\nStep 1 completed...");

    "Step 1 is Ok"
}

async fn completed_2() -> &'static str {

    for _i in 0..250 {
       
        sleep(Duration::from_millis(10)).await;
    }

    println!("\nStep 2 completed...");

    "Step 2 is Ok"
}

async fn completed_3() -> &'static str {

    for _i in 0..50 {

        sleep(Duration::from_millis(10)).await;
    }

    println!("\nStep 3 completed...");

    "Step 3 is Ok"
}

// Step 3 completed...

// Step 2 completed...

// Step 1 completed...
