fn main() {

    let names = ["Mustafa", "Ayhan", "Bengü", "Kağan"];

    println!("{:?}", names.get(2)); // Some("Bengü")

    get_name(&names, 2); // His/Her name is Bengü.

    get_name(&names, 22); // Not found!
}

fn get_name(names_array: &[&str], index_val: usize) {

    let n = names_array.get(index_val);

    match n {
        
        Some(n) => println!("His/Her name is {n}."),
        None => println!("Not found!")
    }
}
