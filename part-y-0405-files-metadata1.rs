use std::fs::File;

const FILE_PATH: &str = r"D:\_rust\rust-again\hi\mustafa.txt";

fn main() {

    if let Ok(my_file) =  File::open(FILE_PATH){

        println!("{my_file:?}");

        let meta_data = my_file.metadata().unwrap();

        println!("{meta_data:?}")
    } 

    else {

        println!("File not found!")
    }
}

// File { handle: 0xb4, path: "\\\\?\\D:\\_rust\\rust-again\\hi\\mustafa.txt" }
// Metadata { file_type: FileType { is_file: true, is_dir: false, is_symlink: false, .. }, permissions: Permissions(FilePermissions { attrs: 32 }), len: 14, 
//modified: SystemTime { intervals: 134369714464096612 }, accessed: SystemTime { intervals: 134369714464096612 }, created: SystemTime { intervals: 134369701089158748 }, .. }
