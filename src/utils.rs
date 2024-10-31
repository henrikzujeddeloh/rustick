use terminal_size::{Width, Height, terminal_size};
use std::io::Read;
use std::fs::File;
use std::io::ErrorKind;
use std::fs;


// open file at path (if it doesn't exist, create it) and return file object
pub fn open_file(file_path: &str) -> File {
    match File::open(&file_path) {
        Ok(file) => {
            println!("opened {}", &file_path);
            return file
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create(&file_path) {
                Ok(fc) => { 
                    println!("created {}", &file_path);
                    return fc
                }
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

// read opened file to string
pub fn read_file(file: &mut File) -> String {
    let mut contents = String::new();
    let _ = file.read_to_string(&mut contents);
    //println!("data.json contains: {}", &contents);
    contents
}

// write string to file
pub fn write_file(contents: &String, path: &str) {
    fs::write(&path, &contents).expect("write contents file");
}

pub fn get_term_width() -> u32 {
    if let Some((Width(w), Height(_h))) = terminal_size() {
        return w.into();
    } else {
        println!("Unable to get terminal size");
        return 0;
    }
}
