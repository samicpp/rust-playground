use std::env;
use std::fs;

fn main(){
  let mut args: Vec<String>=env::args().collect();
    let cmd=args[0].clone();
    args.remove(0);

    if args.len()!=2{
        println!("{cmd}: Need 2 arguments");
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

    let mut found:String="".to_string();
    for file in dir {
      //dbg!(file.unwrap().path().display());
      let str=file.unwrap().path().display().to_string();
      //println!("{}\r\n{}",str.replace(&args[1],""),args[1]);
      if str.replace(&args[0],"")==args[1] {
        found=str;
      };
    };
    if !found.is_empty(){
        println!("{found}");
    };

}
