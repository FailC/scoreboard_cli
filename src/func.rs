
pub mod fail{
    use std::{io::{self, Write, BufReader, stdout}, time::Instant};
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufWriter;

    #[derive(Debug)]
    pub struct User {
        name: String,
        score: i32,
    }

    impl User {
        fn new(name: String, number: i32) -> User {
            let new_user = User { name, score: number };
            return new_user
        }

        pub fn create_user() -> User {
            println!("creating user");
            println!("Name: ");
            let name = input_string();
            println!("Score: ");
            let number = input_i32();
            User::new(name, number)
        }
    }

    pub fn print_menu() {
        println!("1): print list");
        println!("2): create user");
        println!("3): set new score");
        println!("4): delete user");
        println!("5): delete all users");
        println!("0): exit");
    }

    pub fn input_i32() -> i32 {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().parse::<i32>().unwrap_or(99)
    }

    pub fn input_string() -> String {
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).unwrap();
        input.trim().to_string()
    }

    pub fn read_file(file: File, list: &mut Vec<User>) -> u64{
        println!("Reading from File:");
        let reader = BufReader::new(&file);
        // Read the lines from the file and parse them into User structs.
        for line in reader.lines() {
            stdout().flush().unwrap();
            if let Ok(line) = line {
                let mut fields = line.split(" ");
                let name = fields.next().unwrap().to_string();
                let score = fields.next().unwrap().parse::<i32>().unwrap();
                list.push(User { name, score });
            }
        }
        let total_lines = list.len();
        total_lines as u64
    }
    
    pub fn save_file(list: &Vec<User>){
        let mut new_file = BufWriter::new(File::create("user.txt").unwrap());
        //let mut new_file = File::create("user.txt").unwrap(); // try this one

        let mut buffer = Vec::new();
        for i in list {
            buffer.extend_from_slice(i.name.as_bytes());
            buffer.extend_from_slice(b" ");
            buffer.extend_from_slice(i.score.to_string().as_bytes());
            buffer.extend_from_slice(b"\n");
        }
        new_file.write_all(&buffer).unwrap();
    }

    pub fn print_list(list: &Vec<User>) {
        if list.is_empty() {
            println!("List is empty!\n");
            return;
        }
        let start = Instant::now();
        let stdout = stdout();
        let mut writer = BufWriter::new(stdout.lock());

        for (index,item) in list.iter().enumerate(){
            writeln!(writer,"{}\t{}\t {}",index+1, item.name, item.score).unwrap();
        }
        writer.flush().unwrap();
        println!();
        let end = Instant::now();
        let time = end - start;
        dbg!(time);
    }

    pub fn sort(list: &mut Vec<User>) {
        let start = Instant::now();
        list.sort_by(|a, b| b.score.cmp(&a.score));
        let end = Instant::now();
        let time = end - start;
        dbg!(time);
    }


    pub fn set_score(list: &mut Vec<User>) {
        print_list(&list);
        print!("User: ");
        stdout().flush().unwrap();
        let index: usize = {
            let mut index = input_i32() as usize;
            dbg!(index);
            if list.len() < index {
                index = list.len();
            }
            index
        };

        print!("Score: ");
        stdout().flush().unwrap();
        let score = input_i32();
        list[index-1].score = score;
        println!();
    }


    pub fn delete_user(list: &mut Vec<User>){
        print_list(list);
        print!("Delete: ");
        io::stdout().flush().unwrap();
        let input = input_i32();
        let mut index = input as usize;
        let lenght = list.len();
        if index >= lenght {
            index = lenght;
        }
        if index <= 0 { index = 1 }
        list.remove(index-1);
    }

    pub fn delete_all_user(list: &mut Vec<User>) {
        if list.is_empty() { 
        println!("List is empty"); 
        return;
        }
        list.clear(); 
        println!("List cleared\n");
    }
} 
