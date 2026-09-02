fn main() {

    let name1 = "Ayhan";
    let name2: String = "Bengü".to_string();
    let name3 = String::from("Yağız");

    greet(name1.to_string());

    println!("{name1}");

    greet(name2);

    //println!("{name2}"); // error[E0382]: borrow of moved value: `name2` 

    greet_ref(&name3);

    println!("{name3}");
}

fn greet(name: String) {

    println!("Hi {name}!")
}

fn greet_ref(name: &String) {

    println!("Hi {name}!")
}

// Hi Ayhan!
// Ayhan
// Hi Bengü!
// Hi Yağız!
// Yağız
