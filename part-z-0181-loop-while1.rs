fn main() {

    let mut age: u8 = 17;

    let mut i = 0;

    while age < 25 {

        println!("Age {}: {}", i + 1, age);

        age += 1;
        i += 1;
    }
}

// Age 1: 17
// Age 2: 18
// Age 3: 19
// Age 4: 20
// Age 5: 21
// Age 6: 22
// Age 7: 23
// Age 8: 24
