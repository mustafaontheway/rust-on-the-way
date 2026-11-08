fn main() {

}

async fn completed_1() -> &'static str {

    "Step 1 is Ok"
}

async fn completed_2() -> &'static str {

    "Step 2 is Ok"
}

async fn completed_3() -> &'static str {

    "Step 3 is Ok"
}

async fn ready_to_go() {
    
    let c1 = completed_1().await;
    let c2 = completed_2().await;
    let c3 = completed_3().await;

    let final_c = format!("C1: {:?} - C2: {:?} - C3: {:?}", c1, c2, c3);

    println!("Al steps completed. Let's go!");

    println!("{final_c}")
}
