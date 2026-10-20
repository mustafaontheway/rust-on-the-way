use std::fs::File;

fn main() {

    let my_file = File::open(r"D:\_rust\rust-again\hi\mustafa.txt").unwrap();

    println!("{my_file:?}")
}

// File { handle: 0xc0, path: "\\\\?\\D:\\_rust\\rust-again\\hi\\mustafa.txt" }
