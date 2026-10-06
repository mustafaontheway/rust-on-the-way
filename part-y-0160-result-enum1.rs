fn main() {

    let ready: Result<bool, &str> = Ok(true);

    let checked: Result<bool, &str> = Err("We haven't checked yet!");

    println!("{ready:?}");

    println!("{checked:?}");
}

// Ok(true)
// Err("We haven't checked yet!")
