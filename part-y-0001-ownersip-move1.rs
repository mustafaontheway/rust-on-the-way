fn main() {

    let name = String::from("Mustafa");

    let my_name = name;

    //println!("{name}"); // error[E0382]: borrow of moved value: `name` 

    println!("{my_name}");
}



