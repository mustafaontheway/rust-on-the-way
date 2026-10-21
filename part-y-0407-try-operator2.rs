use std::fs;

const FILE_PATH: &str = r"D:\_rust\rust-again\hi\mustafa.txt";

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let contents = fs::read_to_string(FILE_PATH)?;

    println!("{contents}");

    Ok(())     
}

