
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

    let count = file_read(file, &mut list);
    println!("Sorting users..");
    list_sort(&mut list);
    println!("Writing to file..");
    file_save(&list); 
    println!("Done!");
    println!("Total users: {}\n", count);
    
    loop {
    print_menu();
    print!("Waiting for input: "); 
    io::stdout().flush().unwrap();
    
    let input: i32 = input_i32();
    println!();

    match input {
        0 => break,

        1 => {
            list_print(&list);
        }
        2 => {
            list.push(User::user_create());
            list_sort(&mut list);
            file_save(&list); 
        }

        3 => {
            println!("set score");
            set_score(&mut list);
            list_sort(&mut list);
            file_save(&list); 
            }


        4 => {
            println!("delete user");
            user_delete(&mut list);
            list_sort(&mut list);
            file_save(&list); 
        },

        5 => {
            println!("deleting list..");
            user_delete_all(&mut list);
        },

        _ => println!("incorrect input"),
    } 

    }
    Ok(()) 
}
