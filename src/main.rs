mod api;
mod parse;
mod stations;

use crate::api::SeptaApi;
use crate::parse::Parse;
use std::env;

fn main() {
    let api = SeptaApi::new();
    let argv: Vec<String> = env::args().skip(1).collect();
    let str_argv: Vec<&str> = argv.iter().map(String::as_str).collect();

    match str_argv.as_slice() {
        ["-h" | "--help", ..] => {
            println!("Usage: tst [OPTIONS] COMMAND [ARGS]...\n\n\
                Options:\n  \
                --help  Show this message and exit.\n\n\
                Commands:\n  \
                  arrivals  Find the next arrivals at a given train station\n  \
                  next      Search for the next train going from an origin to a destination\n  \
                  search    Search for a given station\n  \
                  train     Track a given train using it's number\n");
            std::process::exit(0);
        },
        ["arrivals", station] => {
            for arrival in api.arrivals(station, 5).unwrap().parse().iter() {
                println!("{arrival}");
            }
            std::process::exit(0);
        },
        ["arrivals", station, num] => {
            for arrival in api.arrivals(station, num.parse::<u8>().unwrap_or(5)).unwrap().parse().iter() {
                println!("{arrival}");
            }
            std::process::exit(0);
        },
        ["next", start, end] => {
            for arrival in api.next_to_arrive(start, end, 5).unwrap().parse().iter() {
                println!("{arrival}");
            }
            std::process::exit(0);
        },
        ["next", start, end, num] => {
            for arrival in api.next_to_arrive(start, end, num.parse::<u8>().unwrap_or(5)).unwrap().parse().iter() {
                println!("{arrival}");
            }
            std::process::exit(0);
        },
        ["train", train_num] => {
            for arrival in api.train_schedule(train_num).unwrap().parse().iter() {
                println!("{arrival}");
            }
            std::process::exit(0);
        },
        [..] => {
            println!("Invalid command: use --help or -h for usage details");
            std::process::exit(1);
        }
    }
}
