fn main() {

    let name = String::from("Mustafa");

    drop(name); // memory clean

    //println!("{name}"); // error[E0382]: borrow of moved value: `name`   
}
