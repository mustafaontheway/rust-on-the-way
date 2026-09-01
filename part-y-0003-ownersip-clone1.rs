fn main() {

    let name = String::from("Mustafa");

    let my_name = name.clone();

    drop(name); // memory clean

    println!("{my_name}"); // Mustafa
}



