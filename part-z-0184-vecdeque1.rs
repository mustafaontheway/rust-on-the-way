use std::collections::VecDeque;

// Deque -> Double-Ended Queue
// Enqueue -> adding an element
// Dequeue -> removing an element

fn main() {

    let mut ages: VecDeque<u8> = VecDeque::from([22u8, 30, 1, 99, 77]);

    // Enqueue

    ages.push_front(100);

    ages.push_back(101);

    println!("Ages: {ages:?}");

    println!("------------------------");

    println!("First age: {:?}", ages.front());

    println!("Last age: {:?}", ages.back());

    println!("Ages: {ages:?}");

    println!("------------------------");

    // Dequeue 

    let first_age = ages.pop_front();

    println!("First age: {:?}", first_age);

    let last_age = ages.pop_back();

    println!("Last age: {:?}", last_age);

    println!("Ages: {ages:?}");
}

// Ages: [100, 22, 30, 1, 99, 77, 101]
// ------------------------
// First age: Some(100)
// Last age: Some(101)
// Ages: [100, 22, 30, 1, 99, 77, 101]
// ------------------------
// First age: Some(100)
// Last age: Some(101)
// Ages: [22, 30, 1, 99, 77]
