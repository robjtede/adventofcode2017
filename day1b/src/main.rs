use std::io::prelude::*;
use std::env;
use std::process;
use std::fs::File;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let nums: String = get_seq(&config);

    let answer: u32 = run(&nums);
    
    println!("Answer: {}", answer);
}

fn run(nums: &str) -> u32 {
    let total: u32 = nums.chars().enumerate().fold(0, |acc, (i, n)| {
        let num: u32 = n.to_digit(10).unwrap();
        let comp: u32 = compare_to(nums, i as u32).unwrap();
        
        // println!("{:?}, {:?}", num, comp);
        
        if num == comp {
            return acc + num;
        } else {
            return acc;
        }
    });

    return total;
}

struct Config {
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        let path = args[1].clone();

        return Ok(Config { path });
    }
}

fn get_seq(config: &Config) -> String {
    let mut file: File = match File::open(&config.path) {
        Err(why) => {
            println!("Problem opening file: {}", why);
            process::exit(1);
        }

        Ok(file) => file,
    };

    let mut contents: String = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => (),

        Err(why) => {
            println!("{:?}", why);
        }
    };
    
    return contents.trim().to_string();
}

fn compare_to(seq: &str, pos: u32) -> Result<u32, &str> {
    if seq.len() % 2 == 1 {
        return Err("sequence is odd in length");
    }

    // better part 1 solution
    // let idx: usize = (pos + 1) as usize % seq.len();
    
    let idx: usize = (pos + (seq.len() as u32 / 2)) as usize % seq.len();
    let opp: u32 = seq.chars().nth(idx).unwrap().to_digit(10).unwrap();

    return Ok(opp);
}
