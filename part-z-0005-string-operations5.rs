fn main() {

    let mut full_name = "    Büyükdereli Mustafa Kültigin         ";

    println!("{}", full_name.len());

    full_name = full_name.trim_end();

    println!("{}", full_name.len());
}

// 44
// 35
