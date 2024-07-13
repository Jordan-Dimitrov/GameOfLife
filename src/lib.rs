use std::env;

pub struct Config {
    pub width: i32,
    pub height: i32
}

impl Config{
    pub fn new(mut args: env::Args) -> Result<Config, &'static str>{
        args.next();

        let width:i32 = match  args.next()
        {
            Some(x) => match x.parse(){
                Ok(x) => x,
                Err(_) => return Err("Invalid type")
            },
            None => return Err("No width passed"),
        };

        let height:i32 = match  args.next()
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