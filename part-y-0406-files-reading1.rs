use std::fs::File;

use std::io::Read;

const FILE_PATH: &str = r"D:\_rust\rust-again\hi\mustafa.txt";

fn main() {

    if let Ok(mut my_file) =  File::open(FILE_PATH) {

        println!("{my_file:?}");

        let mut contents = String::new();

        my_file.read_to_string(&mut contents).unwrap();

        println!("File content: {contents}")
    } 

    else {

        println!("File not found!")
    }
}

// File content: lorem ipsum...
