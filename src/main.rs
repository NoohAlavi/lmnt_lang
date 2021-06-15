mod lmnt;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        println!("Please enter a file to run as a parameter");
        return;
    }
    
    if lmnt::run(&args[1]).is_err() {
        println!("rip");
    }
}
