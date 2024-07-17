use crossterm::event::KeyCode;

#[derive(Clone, Copy, PartialEq)]
pub enum Status{
    Alive,
    Dead
}

pub enum GameState{
    Paused,
    Running,
    Stopped,
    SpeedUp,
    SpeedDown
}

impl GameState{
    pub fn new(input : KeyCode) -> Self{
        match input {
            KeyCode::Char('s') => GameState::Stopped,
            KeyCode::Char('p') => GameState::Paused,
            KeyCode::Char('u') => GameState::SpeedUp,
            KeyCode::Char('d') => GameState::SpeedDown,
            _ => GameState::Running,
        }
    }
}

impl Status{
    pub fn new(num: i32) -> Self{
        match num
        {
            x if x <= 0 => Status::Dead,
            _ => Status::Alive
        }
    }
}