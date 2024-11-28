use std::env;
use std::fs;

fn main(){
  let mut args: Vec<String>=env::args().collect();
    let cmd=args[0].clone();
    args.remove(0);

    if args.len()!=1{
        println!("{cmd}: Need 1 argument");
        std::process::exit(1);
    };
    
}
