use std::fs;

const FILE_PATH: &str = r"D:\_rust\rust-again\hi\mustassssssfa.txt";

fn main() {

    //let _ = read_file_content(FILE_PATH);

    let file_txt = match read_file_content(FILE_PATH) {
        
        Ok(content) => content,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };

    println!("File content: {file_txt:?}")
}

fn read_file_content(file_path: &str) -> Result<String, Box<dyn std::error::Error>> {

    let contents = fs::read_to_string(file_path)?;

    //println!("File content: {:?}", contents);

    Ok(contents)
}

// Error: The system cannot find the file specified. (os error 2)
