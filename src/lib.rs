use std::{thread};
use std::io::{Write};
use std::process::exit;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rand::Rng;
use crossterm::{event, QueueableCommand};
use crossterm::event::{Event, KeyEvent, poll};
pub mod config;
pub mod ui;
pub mod state;
pub use config::Config;
pub use ui::clear_console;
pub use state::*;

pub struct Game{
    matrix : Vec<Vec<Status>>,
    speed : u64,
    state: Arc<Mutex<GameState>>
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
            speed: 400,
            state: Arc::new(Mutex::new(GameState::Running))
        }
    }

    pub fn run(&mut self){
        let state1 = Arc::clone(&self.state);
        let state2 = Arc::clone(&self.state);

        thread::spawn(move ||{
            loop {
                let arc_state = Arc::clone(&state1);

                let mut state = arc_state.lock().unwrap();

                if poll(Duration::from_millis(50)).unwrap() {
                    if let Event::Key(KeyEvent { code, modifiers: _, .. }) = event::read().unwrap() {
                        *state = GameState::new(code);
                    }
                }
            }
        });
        loop {
            let arc_state = Arc::clone(&state2);

            let mut state = arc_state.lock().unwrap();

            match *state
            {
                GameState::Stopped => exit(0),
                GameState::Paused =>
                    {
                        drop(state);
                        continue;
                    },
                GameState::SpeedUp => {
                    *state = GameState::Running;
                    if self.speed >= 100 {
                        self.speed -= 100
                    }
                },
                GameState::SpeedDown => {
                    *state = GameState::Running;
                    self.speed += 100
                },
                GameState::Running => (),
            }

            drop(state);

            if !self.check_neighbors(){
                break;
            }

            self.print();
            thread::sleep(Duration::from_millis(self.speed));
        }

        self.print();
        println!("No alive cells left")
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