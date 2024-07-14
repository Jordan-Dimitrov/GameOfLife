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

    pub fn run(&mut self){
        let character = "-";
        let repeated = character.repeat(self.matrix[0].len());
        println!("{}", repeated);

        while self.check_neighbors() {
            print!("\x1B[2J\x1B[1;1H");
            &self.print();
            println!("{}", repeated);
            thread::sleep(Duration::from_millis(400));
        }
        println!("No alive cells lefr")
    }

    pub fn check_neighbors(&mut self) -> bool {
        let height = self.matrix.len();
        let width = self.matrix[0].len();
        let mut next_gen = self.matrix.clone();
        let mut any_alive = false;

        for i in 0..height {
            for j in 0..width {
                let mut alive = 0;

                for f in [-1, 0, 1].iter() {
                    for d in [-1, 0, 1].iter() {
                        if *f == 0 && *d == 0 {
                            continue;
                        }

                        let ni = i as isize + f;
                        let nj = j as isize + d;

                        if ni >= 0 && nj >= 0 && ni < height as isize && nj < width as isize {
                            alive += self.matrix[ni as usize][nj as usize];
                        }
                    }
                }

                let current = self.matrix[i][j];

                if current == 1 && (alive < 2 || alive > 3) {
                    next_gen[i][j] = 0;
                } else if current == 0 && alive == 3 {
                    next_gen[i][j] = 1;
                } else {
                    next_gen[i][j] = current;
                }

                if next_gen[i][j] == 1 {
                    any_alive = true;
                }
            }
        }

        self.matrix = next_gen;
        any_alive
    }
}