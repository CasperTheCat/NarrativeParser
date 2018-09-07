use std;
use std::io::Read;
use parser;
use nom;

fn internal_lex(_x: &Vec<u8>) -> Vec<String>
{
    let mut outVec = Vec::new();
    let mut builder = String::new();


    for &item in _x
    {
        if item as char == ' '
        {
            outVec.push(builder);
            builder = String::new();
        }
        else
        {
            builder.push(item as char);
        }
        //println!("{}",item);
    }
    
    return outVec;
}

pub fn lex(_path: &std::path::Path)
{
    println!("Ingesting File {:?}", _path);
    
    // Open the file
    let mut file_handle = std::fs::File::open(_path).expect("File not found");

    let mut contents = &mut Vec::new();
    file_handle.read_to_end(&mut contents).expect("Read Error");

    let symbols = internal_lex(contents);
    parser::parse(contents);

    for i in &symbols{
        println!("{:?}", i);
    }


}