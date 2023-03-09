
mod func;
use std::{path::Path, fs::OpenOptions, vec, io};
use std::io::Write;
use crate::func::fail::*;


fn main() -> std::io::Result<()>{

    let mut list: Vec<User> = vec![];
    let path = Path::new("user.txt");

    let file = match OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&path) {
            Ok(file) => file,
            Err(e) => panic!("{}",e),
        };

    let count = read_file(file, &mut list);
    println!("Sorting users..");
    sort(&mut list);
    println!("Writing to file..");
    save_file(&list); 
    println!("\nDone!");
    println!("Total users: {}", count);
    
    loop {
    print_menu();
    print!("Waiting for input: "); 
    io::stdout().flush().unwrap();
    
    let input: i32 = input_i32();
    println!();

    match input {
        0 => break,

        1 => {
            //User::print_list(&list);
            print_list(&list);
        }
        2 => {
            list.push(User::create_user());
            sort(&mut list);
            save_file(&list); 
        }

        3 => {
            println!("set score");
            set_score(&mut list);
            sort(&mut list);
            save_file(&list); 
            }


        4 => {
            println!("delete user");
            delete_user(&mut list);
            sort(&mut list);
            save_file(&list); 
        },

        5 => {
            println!("deleting list..");
            delete_all_user(&mut list);
        },

        _ => println!("incorrect input"),
    } 

    }
    Ok(()) 
}
