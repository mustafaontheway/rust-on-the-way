use std::fs::File;

const FILE_PATH: &str = r"D:\_rust\rust-again\hi\mustafa.txt";

fn main() {

    if let Ok(my_file) =  File::open(FILE_PATH){

        println!("{my_file:?}")
    } 

    else {

        println!("File not found!")
    }
}

// File { handle: 0xc0, path: "\\\\?\\D:\\_rust\\rust-again\\hi\\mustafa.txt" }
