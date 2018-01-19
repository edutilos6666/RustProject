use std::io::stdin;
use std::fs::File;
use std::io::{Read, BufReader, Write, BufWriter};

pub struct M {}

impl M {
    pub fn example1(&self) {
        println!("<<example1()>>");
        'outer: loop {
            let number:i32 = 10 ;
            println!("Pick a Number");
            loop {
                let mut line = String::new();
                let input = stdin().read_line(&mut line);
                let guess:Option<i32> = input.ok().map_or(None, |_| line.trim().parse().ok());
                match guess  {
                    None => println!("enter a Number"),
                    Some(n) if n == number => {
                        println!("You Guessed It");
                        break 'outer;
                    }
                    Some(n) if n < number => println!("Too Low"),
                    Some(n) if n > number => println!("Too High"),
                    Some(_) => println!("Error")
                }
            }
        }  // end of 'outer loop
        println!();
    }

    pub fn example2(&self) {
        println!("<<example2()>>");
        let mut id = String::new() ;
        let mut name = String::new();
        let mut age = String::new() ;
        let mut wage = String::new();
        let mut active = String::new();
        println!("Enter your id: ");
        stdin().read_line(&mut id);
        println!("Enter your name: ");
        stdin().read_line(&mut name);
        println!("Enter your age: ");
        stdin().read_line(&mut age);
        println!("Enter your wage: ");
        stdin().read_line(&mut wage);
        println!("Enter your active: ");
        stdin().read_line(&mut active);

        println!("<<You entered>>");
        println!("id = {}", id.trim());
        println!("name = {}", name.trim());
        println!("age = {}", age.trim());
        println!("wage = {}", wage.trim());
        println!("active = {}", active.trim());
        println!();
    }

    pub fn example3(&self) {
        let pathName = "test.dat";
        let data = "foo\nbar\nbim";
        {
        let fh = File::create(pathName).expect("Unable to create new file");
        let mut writer = BufWriter::new(fh);
        writer.write_all(data.as_bytes()).expect("Unable to write data into file");
        }
        let mut data = String::new();
        {
            let fh = File::open(pathName).expect("Unable to open file");
            let mut reader = BufReader::new(fh);
            reader.read_to_string(&mut data).expect("Unable to read data from file");
            println!("Data = {}", data);
        }
    }
}
