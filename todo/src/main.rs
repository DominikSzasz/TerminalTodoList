use std::env;
use std::fs::File;
use std::io::Write;
use std::fs::OpenOptions;
use std::fs::{read_to_string, write};

fn init_list() {
    match File::create(".todo") {
        Ok(_) => println!("Todo List Created"),
        Err(e) => eprintln!("Error creating Todo List: {}", e),
    }
}

fn add_el(arg: &str) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(".todo")
        .unwrap();

    if let Err(e) = writeln!(file, "{}", arg) {
        eprintln!("Couldn't write to file: {}", e);
    }
}

fn list() {
    if let Ok(content) = read_to_string(".todo") {
        println!("\nTodo List:");
        for (line_number, line) in content.lines().enumerate() {
            if line.starts_with("-"){
                println!("{}: \x1B[9m{}\x1B[0m", line_number+1, &line[1..]);
            }
            else {
                println!("{}: {}", line_number + 1, line);
            }
        }
        println!("\n");
    } else {
        println!("Error reading the file");
    }
}

fn done(line_num: usize) {
    let content = match read_to_string(".todo") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        }
    };

    let modified_content: String = content
        .lines()
        .enumerate()
        .map(|(index, line)| {
            if index == line_num {
                format!("-{}", line)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(e) = write(".todo", modified_content) {
        eprintln!("Error writing to the file: {}", e);
    }
}
fn remove(line_num: usize) {
    let content = match read_to_string(".todo") {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading the file: {}", e);
            return;
        }
    };

    // Exclude the nth line
    let modified_content: String = content
        .lines()
        .enumerate()
        .filter_map(|(index, line)| {
            if index == line_num {
                None 
            } else {
                Some(line.to_string())
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    if let Err(e) = write(".todo", modified_content) {
        eprintln!("Error writing to the file: {}", e);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(inst) = args.get(1) {
       match inst.as_str() {
            "init" => {
                init_list();
            }
         
            "add" => {
                if let Some(arg2) = args.get(2) {
                    add_el(arg2);
                } else {
                    println!("No arguments added");
                }
            }
            "rem" => {
               let line_num_to_modify: usize = match args[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Invalid line number provided");
                        std::process::exit(1);
                    }
                };

                remove(line_num_to_modify-1);   
            }
            "list" => {
                list();
            }
            "done" => {
                let line_num_to_modify: usize = match args[2].parse() {
                    Ok(num) => num,
                    Err(_) => {
                        eprintln!("Invalid line number provided");
                        std::process::exit(1);
                    }
                };

                done(line_num_to_modify-1);   
            }

            "help" => {
            println!("\nUsage:\n - todo init \n - todo add \"item\"\n - todo done <Line Number>\n - todo rem <Line Number>\n - todo list\n");
            }
            _ =>
            {
                println!("Command Not Found");
            }
        }
    }
        
}
