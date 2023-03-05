#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::{self, Read, Write, BufReader, stdout}, string, vec, process::exit};
use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::io::BufWriter;

#[derive(Debug)]
struct User {
    name: String,
    score: i32,
}

impl User {
    fn new(name: String, number: i32) -> User {
        let new_user = User { name, score: number };
        return new_user
    }

    fn print_list(list: &Vec<User>) {
        println!("printing list");
        for i in list{
            println!("{}", i.name);
        }
    }
    fn create_user() -> User {
        println!("creating user");
        println!("Name: ");
        let name = input_string();
        println!("Score: ");
        let number = input_i32();
        User::new(name, number)
    }
}

fn print_menu() {
    println!("(1): print list");
    println!("(2): create user");
    println!("(3): set score//beta");
    println!("(4): delete user");
    println!("(5): delete all users");
    println!("(0): exit");
}

fn input_i32() -> i32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(99)
}

fn input_string() -> String {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn save_file(list: &Vec<User>){
    //let mut new_file = File::create("user.txt").expect("Couldnt create new file in function");
    let mut new_file = File::create("user.txt").unwrap(); // try this one

    for i in list {
        let temp = &i.name;
        let s = &i.score;
        new_file.write_all(temp.as_bytes()).unwrap();
        new_file.write_all(b" ").unwrap();
        new_file.write_all(&s.to_string().as_bytes()).unwrap();
        new_file.write_all(b"\n").unwrap();
    }
}

fn read_file(file: File, list: &mut Vec<User>){
    let reader = BufReader::new(&file);
    // maybe add more error handling later
    for line in reader.lines() {
        if let Ok(line) = line {
            let mut fields = line.split(" ");
            let name = fields.next().unwrap().to_string();
            let score = fields.next().unwrap().parse::<i32>().unwrap();
            list.push(User { name, score });
        }
    }
}
// test
fn print_list(list: &Vec<User>) {
    if list.is_empty() {
        println!("List is empty!\n");
        return;
    }
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());

    for (index,item) in list.iter().enumerate(){
        writeln!(writer,"{}\t{}\t {}",index+1, item.name, item.score).unwrap();
    }
    writer.flush().unwrap();
    //println!(); ??
}

fn sort(list: &mut Vec<User>) {
    list.sort_by(|a, b| b.score.cmp(&a.score));
}
// python print huge numbers of users!!! 
fn delete_user(list: &mut Vec<User>){
    // chose user and call drop 
    //let temp = list.clone();
    print_list(list);
    println!("Delete: ");
    let input = input_i32();
    let mut index = input as usize;
    let lenght = list.len();
    if index >= lenght {
        index = lenght;
    }
    if index <= 0 { index = 1 }
    list.remove(index-1);
}

fn delete_all_user(list: &mut Vec<User>) {
    if list.is_empty() { println!("List is empty"); }
    else  { list.clear(); }
}


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

    read_file(file, &mut list);
    sort(&mut list);
    save_file(&list); 
    
        loop {
        print_menu();
        println!("waiting for input:");
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
            }

            3 => println!("3:set score"),

            4 => {
                println!("4: delete user");
                delete_user(&mut list);
            },

            5 => {
                println!("Deleting list..");
                delete_all_user(&mut list);
            },

            _ => println!("incorrect input"),
        } 
        sort(&mut list);
        save_file(&list); // automatic save
    }
    Ok(()) 
}
