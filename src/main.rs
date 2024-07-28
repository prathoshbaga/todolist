use std::env;
use todolist;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len()>1{
        let command = &args[1];
        match command.as_str(){
            "create" => todolist::create(),
            "list" => todolist::list(),
            "add" => todolist::add(&args[2..]),
            "rm" => todolist::remove(&args[2..]),
            "done" => todolist::done(&args[2..]),
            "reset" => todolist::reset(),
            "help" | "--help" | "-h" => todolist::help(),
            _ => println!("please enter help or --help or -h for more info")
        }
    } else{
        println!("please enter help or --help or -h for more info")
    }
}
