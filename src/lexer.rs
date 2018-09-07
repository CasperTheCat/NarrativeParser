use std;
use std::io::Read;

fn internal_lex(_x: &Vec<u8>) -> Vec<String>
{

}

pub fn lex(_path: &std::path::Path)
{
    println!("Ingesting File {:?}", _path);
    
    // Open the file
    let mut file_handle = std::fs::File::open(_path).expect("File not found");

    let mut contents = &mut Vec::new();
    file_handle.read_to_end(&mut contents).expect("Read Error");

    internal_lex(contents);
}