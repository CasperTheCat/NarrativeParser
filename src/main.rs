#[macro_use]
extern crate nom;

use nom::{Needed, Context};
use std::io::prelude::*;

#[derive(Debug, PartialEq)]
struct SyntaxNode {
    subjectNoun: String,
    actingVerb: String,
    targetNoun: String,
    sugar: String
}

/*enum Err<I, E = u32> {
  Incomplete(Needed),
  Error(Context<I, E>),
  Failure(Context<I, E>),
}*/

/*type IResult<I,O,E = u32> = Result<(I,O), Err<I,E>>;*/

use nom::IResult;


fn parseSubjectNoun(x: &[u8]) -> IResult<&[u8], String>
{
    let mut builder = String::new();

    for xi in x.iter()
    {
        //let string_xi = std::str::from_utf8(xi).expect("Unable to utf8 encode");

        //println!("{:?}", string_xi);

        let char_xi = xi as char;

        if char_xi == ' '
        {
            break;
        }
        else
        {
            builder.push(char_xi);
        }
    }
    
    if builder == ""
    {
        return Err(nom::Err::Error("Wat"));
    }
    else
    {
        return Ok((x, builder));
    }
}

fn parseActingVerb(x: &[u8]) -> IResult<&[u8], String>
{
    Ok((x, "Meets".to_string()))
}

fn parseTargetNoun(x: &[u8]) -> IResult<&[u8], String>
{
    Ok((x, "Ally".to_string()))
}

fn parseSugar(x: &[u8]) -> IResult<&[u8], String>
{
    Ok(("", String::from_utf8(x).expect("Failed to convert Sugar")))
}

named!(nfd_parse<&[u8], SyntaxNode>,
    alt!
    (
        do_parse!
        (
            aa: parseSubjectNoun >>
            bb: parseActingVerb >>
            cc: parseTargetNoun >>
            dd: parseSugar >>
            (SyntaxNode{subjectNoun: aa.to_string(), actingVerb: bb.to_string(), targetNoun: cc.to_string()})
        )
        |
        do_parse!
        (
            (SyntaxNode{subjectNoun: "Ally".to_string(), actingVerb: "Betrays".to_string(), targetNoun: "Player".to_string()})
        )
    )
);

fn parse(_path: &std::path::Path)
{
    println!("Ingesting File {:?}", _path);
    
    // Open the file
    let mut file_handle = std::fs::File::open(_path).expect("File not found");

    let mut contents = &mut Vec::new();
    //let mut contents = String::new();
    file_handle.read_to_end(&mut contents).expect("Read Error");
    //file_handle.read_to_string(&mut contents).expect("Read Error");

    /*let s = match std::str::from_utf8(contents)
    {
        Ok(v) => v,
        Err(e) => panic!("Invalid utf8"),
    };*/

    let parsed = nfd_parse(contents);


    println!("{:?}", parsed);
}

fn process_queue(queue: &mut std::collections::VecDeque<std::path::PathBuf>)
{
    while !queue.is_empty()
    {
        let path = queue.pop_front();
        if path.is_none()
        {
            println!("WTF?");
        }

        let unwrapped_path = path.unwrap();
        let path_full = unwrapped_path.as_path();

        if path_full.is_file()
        {
            parse(&path_full);
        }
        else
        {
            for item in path_full.read_dir().expect("Directory read failure")
            {
                //queue.push_back(item);
                if let Ok(item) = item
                {
                    let clone = item.path().clone();
                    queue.push_back(clone);
                }
            }

        }
    }
}

fn main()
{
    // firstly, we want to check out argv[1]
    if std::env::args().len() == 1
    {
        println!("HelloWorld");
        return;
    }

    let args: Vec<String> = std::env::args().collect();

    println!("Parsing {:?}", args[1]);

    let path = std::path::Path::new(&args[1]);

    // Create a queue of all the things
    let mut f_queue = std::collections::VecDeque::new();

    f_queue.push_back(path.to_path_buf());

    process_queue(&mut f_queue);
}