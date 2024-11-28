use std::env;

fn main(){
    std::process::exit(_main());
}

fn _main()->i32{
    let mut args:Vec<String>=env::args().collect();
    args.remove(0);
    if(args.len()<1){
        println!("hello nameless");
        return 1;
    }
    println!("hello, {}",args[0]);

    return 0;
}