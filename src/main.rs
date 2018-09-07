#[macro_use]
extern crate nom;
mod lexer;
mod parser;

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
            lexer::lex(&path_full);
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