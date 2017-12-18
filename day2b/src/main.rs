use std::io::prelude::*;
use std::env;
use std::process;
use std::fs::File;
use std::collections::BTreeSet;

fn main() {
    let config: Config = Config::new(env::args()).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    let input = get_input(&config);

    let chksums: Vec<i32> = input
        .iter()
        .map(|line| { return line_chksum(&line); })
        .collect();

    let sum = chksums.iter().sum::<i32>();

    println!("Answer: {:?}", sum);
}

fn line_chksum(line: &Vec<i32>) -> i32 {
    let mut multiples: BTreeSet<(i32, i32)> = BTreeSet::new();
    let lrgst = *line.iter().max().unwrap();

    for cell in line.iter() {
        let mut i = 1;

        while cell * i <= lrgst {
            i += 1;
            multiples.insert((*cell, cell * i));
        }
    }

    for cell in line.iter() {
        for multiple in multiples.iter() {
            if *cell == multiple.1 {
                return (multiple.1 / multiple.0) as i32;
            }
        }
    }

    panic!("input should always have a divisible pair");
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
        .map(|line| {
            return line.split('\t')
                .map(|x| x.to_string().parse().unwrap())
                .collect();
        })
        .collect();

    return spreadsheet;
}

struct Config {
    path: String,
}

impl Config {
    fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        }

        args.next();

        let path = args.next().unwrap();

        return Ok(Config { path });
    }
}
