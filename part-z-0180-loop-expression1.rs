fn main() {

    let mut age: u8 = 17;

    let oldest = loop {

        if age > 23 {

            break age;
        }

        age += 1;
    };

    println!("{oldest}") // 24
}

