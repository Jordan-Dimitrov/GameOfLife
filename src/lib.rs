use std::{env, thread};
use std::time::Duration;
use rand::Rng;

pub struct Config {
    pub width: usize,
    pub height: usize
}

impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();

        let width:usize = match  args.next()
        {
            Some(x) => match x.parse(){
                Ok(x) => x,
                Err(_) => return Err("Invalid type")
            },
            None => return Err("No width passed"),
        };

        let height:usize = match  args.next()
        {
            Some(x) => match x.parse(){
                Ok(x) => x,
                Err(_) => return Err("Invalid type")
            },
            None => return Err("No height passed"),
        };

        Ok(Config {
            width,
            height
        })
    }
}

pub struct Game{
    matrix : Vec<Vec<i32>>
}

impl Game{
    pub fn new(config: Config) -> Game{
        let mut matrix: Vec<Vec<i32>> = vec![];

        for _ in 0..config.height {
            let mut row = vec![0; config.width];

            for i in 0..row.len() {
                let secret_number = rand::thread_rng().gen_range(0..=1);
                row[i] = secret_number;
            }

            matrix.push(row);
        }

        Game{
            matrix
        }
    }

    pub fn print(&self){
        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {
                print!("{0}", self.matrix[i].get(j).unwrap())
            }
            println!()
        }
    }

    pub fn run(&self){
        while true {
            print!("\x1B[2J\x1B[1;1H");
            &self.print();

            thread::sleep(Duration::from_millis(400));
        }
    }

    pub fn check_neighbors(&mut self){
        let old_gen = &self.matrix.clone();

        for i in 0..self.matrix.len() {
            for j in 0..self.matrix[i].len() {

            }
        }
    }
}