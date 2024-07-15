use std::{thread};
use std::io::{stdout, Write};
use std::time::Duration;
use rand::Rng;
use crossterm::{ExecutableCommand, QueueableCommand};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::terminal::{Clear, ClearType};

pub mod config;
pub use config::Config;

fn clear_console() {
    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}
#[derive(Clone, Copy, PartialEq)]
enum Status{
    Alive,
    Dead
}

impl Status{
    fn new(num: i32) -> Self{
        if num != 0 {
            Status::Alive
        }
        else{
            Status::Dead
        }
    }
}

pub struct Game{
    matrix : Vec<Vec<Status>>,
    speed : u64
}

impl Game{
    pub fn new(config: Config) -> Game{
        let mut matrix: Vec<Vec<Status>> = vec![];

        for _ in 0..config.height {
            let mut row : Vec<Status> = vec![];

            for _ in 0..config.width {
                let secret_number = rand::thread_rng().gen_range(0..=1);
                row.push(Status::new(secret_number));
            }

            matrix.push(row);
        }

        Game{
            matrix,
            speed: 400
        }
    }

    pub fn print(&self){
        clear_console();

        let character = "@";
        let repeated = character.repeat(self.matrix[0].len() + 2);
        println!("{}", repeated);

        for i in 0..self.matrix.len() {
            print!("{0}", character);
            for j in 0..self.matrix[i].len() {
                let symbol = match self.matrix[i][j]{
                    Status::Alive => "█",
                    Status::Dead => "."
                };
                print!("{0}", symbol);
            }
            print!("{0}", character);
            println!()
        }
        println!("{}", repeated);
    }

    pub fn run(&mut self){
        while self.check_neighbors() {
            self.print();
            thread::sleep(Duration::from_millis(self.speed));
        }

        self.print();
        println!("No alive cells lefr")
    }

    fn check_neighbors(&mut self) -> bool {
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

                        if ni > 0 && nj > 0 && ni < height as isize && nj < width as isize {
                            alive += match self.matrix[ni as usize][nj as usize]
                            {
                                Status::Alive => 1,
                                Status::Dead => 0
                            };
                        }
                    }
                }

                let current = self.matrix[i][j];

                if current == Status::Alive && (alive < 2 || alive > 3) {
                    next_gen[i][j] = Status::Dead;
                } else if current == Status::Dead && alive == 3 {
                    next_gen[i][j] = Status::Alive;
                } else {
                    next_gen[i][j] = current;
                }

                if next_gen[i][j] == Status::Alive {
                    any_alive = true;
                }
            }
        }

        self.matrix = next_gen;
        any_alive
    }
}