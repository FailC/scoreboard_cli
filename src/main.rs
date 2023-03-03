#![allow(dead_code)]
#![allow(unused_imports)]

use std::{io::{self, Read, Write, BufReader}, string, vec};
use std::path::Path;
use std::fs::File;
use std::io::BufRead;
use std::io::prelude::*;
use std::fs::OpenOptions;

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
        let name = input_name();
        println!("Score: ");
        let number = input();
        User::new(name, number)
    }
}

fn print_menu() {
    println!("(1): print list");
    println!("(2): create user");
    println!("(3): set score//beta");
    println!("(0): exit");
}

fn input() -> i32 {
    let mut input: String = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i32>().unwrap_or(99)
}

fn input_name() -> String {
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

fn print_list(list: &Vec<User>) {
    for (index,item) in list.iter().enumerate(){
        println!("{}\t{}\t {}",index+1, item.name, item.score);
    }
    println!();
}

fn sort(list: &mut Vec<User>) {
    list.sort_by(|a, b| b.score.cmp(&a.score));
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
            Err(_) => panic!("Can't do shit"),
        };


    read_file(file, &mut list);
    
        loop {
        print_menu();
        println!("waiting for input:");
        let input: i32 = input();
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

            _ => println!("incorrect input"),
        } 
        sort(&mut list);
        save_file(&list); // automatic save
    }
    Ok(()) 
}



 

