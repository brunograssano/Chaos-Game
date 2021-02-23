mod app;
mod chaos_game;

use crate::app::App;
use std::env;
extern crate getopts;
use getopts::{Options, Matches};

fn print_usage(opts: Options) {
    let brief = format!("Usage: chaos_game [options]");
    print!("{}", opts.usage(&brief));
}

fn parse_arguments() -> Matches{
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("v", "vertices", "Set the number of starting vertices", "3");
    opts.optflag("c", "color", "Uses only one color");
    opts.optflag("h", "help", "Prints this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    if matches.opt_present("h") {
        print_usage(opts);
    }

    matches
}

fn main() {
    let matches = parse_arguments();
    if matches.opt_present("h") {
        return;
    }

    let mut starting_vertices : usize = 0;
    if matches.opt_present("v") {
        if let Some(v) = matches.opt_str("v"){
            starting_vertices = v.parse::<usize>().unwrap();
        }
    }

    let only_one_color = matches.opt_present("c");

    let mut app = App::new(starting_vertices,only_one_color);
    app.game_loop()
}