use std::path::{PathBuf,Path};
use std::fs;
use std::collections::HashSet;


pub fn get_list_file_path() -> Option<PathBuf>{
    dirs::home_dir().map(|mut path|{
        path.push("list.txt");
        path
    })
}


fn read_list() -> Result<String,String>{

    let file_path = get_list_file_path().ok_or_else(||String::from("Unable to find list"))?;

    if !Path::new(&file_path).exists(){
        return Err(String::from("List not yet created, use create cmd first"));
    }

    fs::read_to_string(&file_path).map_err(|e|format!("{}",e))

}


fn write_list(content: String) -> Result<(),String>{

    let file_path = get_list_file_path().ok_or_else(||String::from("Unable to find list"))?;

    if !Path::new(&file_path).exists(){
        return Err(String::from("List not yet created, use create cmd first"));
    }

    fs::write(&file_path,content).map_err(|e|format!("{}",e))

}

fn get_number(args: &[String]) -> Option<Vec<usize>> {
    let mut arg_vec: Vec<usize> = vec![];
    for arg in args{
        if let Ok(number) = arg.parse::<usize>(){
            arg_vec.push(number);
        }
    }
    if arg_vec.len()>0{
        Some(arg_vec)
    } else {
        None
    }
}

fn process_lines<F>(process_line:F) -> Result<(),String>
where
    F: Fn(usize,&str) -> Option<String>
{

    read_list()
    .map(|content|{
        if content.lines().count() == 0 {
            return Err(String::from("Nothing to complete. List empty"));
        } else {
            let new_content: String = content
            .lines()
            .enumerate()
            .filter_map(|(i,line)|process_line(i,line))
            .collect::<Vec<String>>()
            .join("\n");

            let _ = write_list(new_content)
            .map(|_|{
                let _ = list().map_err(|e| e.to_string());
            })
            .map_err(|e| e.to_string());
            Ok(())
        }
    })
    .map_err(|e| e.to_string())?
}


pub fn create() -> Result<(),String> {

    let file_path = get_list_file_path().ok_or_else(|| String::from("Unable to find list"))?;

    if Path::new(&file_path).exists(){
        println!("list already exists");
        Ok(())
    } else {
        fs::File::create(&file_path)
        .map(|_| ())
        .map_err(|err| err.to_string())
    }

}

pub fn list() -> Result<(),String>{

    read_list().map(|content|{
        println!("{}",content);
        ()
    })
    .map_err(|e| e.to_string())
    

}

pub fn add(args:&[String]) -> Result<(),String>{

    read_list()
    .map(|mut content|{
        for arg in args{
            if !content.is_empty(){
                content.push_str("\n");
            }
            content.push_str(arg);
        }
        let _ = write_list(content)
        .map(|_|{
            let _ = list().map_err(|e| e.to_string());
        })
        .map_err(|e| e.to_string());
        
    })
    .map_err(|e| e.to_string())

}

pub fn reset() -> Result<(),String>{
    let file_path = get_list_file_path().ok_or_else(|| String::from("Unable to find list"))?;

    if Path::new(&file_path).exists(){
        let content = "".to_string();
        let _ = write_list(content)
        .map_err(|e| e.to_string());
        Ok(())
    } else {
        Err(String::from("List not yet created, use create cmd first"))
    }
}


pub fn remove(args:&[String]) -> Result<(),String>{

    if let Some(vec) = get_number(&args){
        let lines_hash: HashSet<usize> = vec.iter().map(|&x : &usize| x-1).collect();
        let _ = process_lines(|i,line|{
            if !lines_hash.contains(&i){
                Some(line.to_string())
            } else {
                None
            }})
        .map_err(|e| e.to_string())?;
        Ok(())

    } else {
        Err(String::from("Indexes invalid. Provide numbers"))
    }
    

}

pub fn done(args:&[String]) -> Result<(),String>{
    
    if let Some(vec) = get_number(&args){
        let lines_hash: HashSet<usize> = vec.iter().map(|&x : &usize| x-1).collect();
        let _ = process_lines(|i,line|{
            if lines_hash.contains(&i){
                Some(format!("** {} ** -> done", line))
            } else {
                Some(line.to_string())
            }})
        .map_err(|e| e.to_string());
        Ok(())

    } else {
        Err(String::from("Indexes invalid. Provide numbers"))
    }

}

pub fn help() {
    println!("{}", TODO_HELP);
}

const TODO_HELP: &str = "Usage: tdl [COMMAND] [ARGUMENTS]
tdl is a simple task list programmed using rust
commands:
    - add [TASK/s]
        adds new task/s
        Example: tdl add workout read sleep
    - list
        lists all tasks
        Example: tdl list
    - done [INDEX/s]
        marks task/s as done
        Example: tdl done 4 5 6  (marks 4th, 5th and 6th tasks as completed)
    - rm [INDEX/s]
        removes a task/s
        Example: todo rm 4 (renoves the fourth task on the list)
    - reset
        deletes all tasks
";


