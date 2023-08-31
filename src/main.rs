use std::env;
use std::process;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;



fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searhing for {}",config.query);

    println!("In file {}", config.filename);

    if let Err(e) = run(config) {
        println!("Apllicaditon error: {}", e);
        process::exit(1);
    } 

}
// myresult を定義
pub type Myresult = std::result::Result<Config, &'static str>;


//config struct
pub struct Config {
    query: String,
    filename: String,
}

impl Config {

    //new config
    fn new(args: &[String]) -> Myresult {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }

}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;


    println!("With text:\n{}", contents);
    Ok(())
}