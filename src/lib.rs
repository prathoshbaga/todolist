
mod todo;


pub fn help() {
    todo::help();
}

pub fn create(){
    todo::create().unwrap_or_else(|e|{
        println!("Error creating {}",e);
    });
}

pub fn list(){
    todo::list().unwrap_or_else(|e|{
        println!("Error reading {}",e);
    });
}

pub fn add(args:&[String]){

    if args.len() > 0 {
        todo::add(&args).unwrap_or_else(|e|{
            println!("Error adding {}",e);
        });
    } else {
        println!("Nothing to add");
    }
}

pub fn remove(args:&[String]){

    if args.len() > 0 {
        todo::remove(&args).unwrap_or_else(|e|{
            println!("{}",e);
        });
    } else {
        println!("Nothing to remove");
    }
}

pub fn done(args:&[String]){

    if args.len() > 0 {
        todo::done(&args).unwrap_or_else(|e|{
            println!("{}",e);
        });
    } else {
        println!("Nothing to complete");
    }
}

pub fn reset(){
    todo::reset().unwrap_or_else(|e|{
        println!("Error reading {}",e);
    });
}
