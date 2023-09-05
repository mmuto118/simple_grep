use std::error::Error;
use std::fs::File;
use std::io::prelude::*;


// myresult を定義
pub type Myresult = std::result::Result<Config, &'static str>;


//config struct
pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {

    //new config
    pub fn new(args: &[String]) -> Myresult {
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let filename = args[2].clone();

        Ok(Config{query, filename})
    }

}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut f = File::open(config.filename)?;
    let mut contents = String::new();

    f.read_to_string(&mut contents)?;


    println!("With text:\n{}", contents);
    Ok(())
}