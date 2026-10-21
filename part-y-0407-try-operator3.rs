use std::fs;

const FILE_PATH: &str = r"D:\_rust\rust-again\hi\mustasfa.txt"; // I've added char 's'

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let contents = fs::read_to_string(FILE_PATH)?;

    println!("{contents}");

    Ok(())     
}

// Error: Os { code: 2, kind: NotFound, message: "The system cannot find the file specified." }
// error: process didn't exit successfully: `target\debug\hi.exe` (exit code: 1)
