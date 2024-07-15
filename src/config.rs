use std::env;

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
