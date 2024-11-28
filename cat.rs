use std::env;
use std::fs;

fn main(){
    let mut args: Vec<String>=env::args().collect();
    let cmd=args[0].clone();
    args.remove(0);

    if args.len()<1{
        println!("{cmd}: Need atleast 1 argument");
        std::process::exit(1);
    };

    for i in args.iter(){
        let text=match fs::read_to_string(&i)/*.expect("couldnt read file")*/{
            Ok(f)=>f,
            Err(err)=>{
                println!("{cmd}: {err}");
                std::process::exit(2);
            }
        };
        print!("{text}");
    }
}