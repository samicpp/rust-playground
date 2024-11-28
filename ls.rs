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

    //println!("{}",args[0]);
    let dir=match fs::read_dir(&args[0]){
      Ok(r)=>r,
      Err(e)=>{
        println!("{cmd}: {e}");
        std::process::exit(2);
      }
    };

    for file in dir {
      //dbg!(file.unwrap().path().display());
      println!("{}", file.unwrap().path().display());
    }

}
