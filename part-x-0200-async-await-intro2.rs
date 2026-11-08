use std::time::Duration;
use std::thread;

fn main() {

    futures::executor::block_on(ready_to_go());
}

async fn completed_1() -> &'static str {

    for i in 0..500 {
        print!("{i} ");
        thread::sleep(Duration::from_millis(10));
    }

    println!("\nStep 1 completed...");

    "Step 1 is Ok"
}

async fn completed_2() -> &'static str {

    for i in 0..500 {
        print!("{i} ");
        thread::sleep(Duration::from_millis(10));
    }

    println!("\nStep 2 completed...");

    "Step 2 is Ok"
}

async fn completed_3() -> &'static str {

    for i in 0..500 {
        print!("{i} ");
        thread::sleep(Duration::from_millis(10));
    }

    println!("\nStep 3 completed...");

    "Step 3 is Ok"
}

async fn ready_to_go() {

    let c3 = completed_3().await;
    
    let c1 = completed_1().await;
    let c2 = completed_2().await;
    

    let final_c = format!("C1: {:?} - C2: {:?} - C3: {:?}", c1, c2, c3);

    println!("Al steps completed. Let's go!");

    println!("{final_c}")
}

// cargo add futures
