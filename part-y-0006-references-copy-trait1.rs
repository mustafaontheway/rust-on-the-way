fn main() {

    let name_heap_value = String::from("Mustafa 1");

    let name_static: &'static str = "Mustafa 2";

    let ref_name_heap_value = &name_heap_value;

    println!("{name_heap_value} is {ref_name_heap_value}");

    let my_name = name_static;

    println!("{name_static} is {my_name}");
}

// Mustafa 1 is Mustafa 1
// Mustafa 2 is Mustafa 2

