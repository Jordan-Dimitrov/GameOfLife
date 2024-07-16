use std::io::{stdout, Write};
use crossterm::cursor::{Hide, MoveTo};
use crossterm::QueueableCommand;
use crossterm::terminal::{Clear, ClearType};
use crate::{Game, Status};

pub fn clear_console() {
    let mut out = stdout();
    out.queue(Hide).unwrap();
    out.queue(Clear(ClearType::All)).unwrap();
    out.queue(MoveTo(0, 0)).unwrap();
    out.flush().unwrap();
}

impl Game{
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
}