use std::env;

pub struct Config {
    pub path: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        args.next();

        let path = args.next().unwrap();

        return Ok(Config { path });
    }
}
