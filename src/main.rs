use std::env;
use std::process;

use minigrep::Config;
fn main() {
    let args: Vec<String> = env::args().collect(); //take args which we give when running program and convert into collection

    let config = Config::new(&args).unwrap_or_else(|err| { //closer is used
        eprintln!("Problem parsing arguments: {}",err); // indicate error
        process::exit(1);
    });
    println!("Searching for {}",config.query);
    println!("In File{}",config.filename);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}",e);
        process::exit(1);
    }

}




