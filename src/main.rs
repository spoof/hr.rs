extern crate terminal_size;
use terminal_size::{Width, Height, terminal_size};
use std::env;
use std::process;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        args.push("#".to_string())
    }

    let size = terminal_size();
    let cols: usize;
    if let Some((Width(c), Height(_))) = size {
        cols = c as usize;
    } else {
        println!("Unable to get terminal size");
        process::exit(1);
    }

    let mut out: String = String::new();
    for (i, c) in args[1].chars().cycle().enumerate() {
        if i >= cols {
            break
        }

        out.push(c)
    }
    println!("{}", out)
}
