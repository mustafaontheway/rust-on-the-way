fn main() {

    // Heap values move ownership on assignment, so referencing (&) is needed to retain the original value

    let name = "Mustafa".to_string();

    let _my_name = &name;

    let _first_name = &name;
}



