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
    
    let input = get_input(&config);
    
    let chksums: Vec<i32> = input.iter().map(|line| {
        return line_chksum(&line);
    }).collect();
    
    println!("Answer: {:?}", chksums.iter().sum::<i32>());
}

fn line_chksum (line: &Vec<i32>) -> i32 {
    let mut lrgst = 0;
    let mut smlst = i32::max_value();
    
    for cell in line.iter() {
        if *cell > lrgst {
            lrgst = *cell
        } else if *cell < smlst {
            smlst = *cell
        }
    }
    
    return lrgst - smlst;
}

fn get_input(config: &Config) -> Vec<Vec<i32>> {
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
    
    let lines: Vec<String> = contents
        .trim()
        .split('\n')
        .map(|x| x.to_string())
        .collect();
        
    let spreadsheet: Vec<Vec<i32>> = lines
        .iter()
        .map(|line| line
            .split('\t')
            .map(|x| x.to_string().parse().unwrap())
            .collect())
        .collect();
    
    return spreadsheet;
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
