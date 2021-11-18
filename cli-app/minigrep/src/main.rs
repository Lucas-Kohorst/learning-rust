use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect(); // grab the args collect them into a vector

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);

    run(config); 
}

// Reads the file
fn run(config: Config) {
    let contents = fs::read_to_string(config.filename)
    .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

// defines the config of the arguments  
struct Config {
    query: String,
    filename: String,
} // defining a struct to hold the config

impl Config { // since this is now an implementation of the Config struct we can call it directly as a type
    fn new (args: &[String]) -> Result<Config, &str> {
        // checking index out of bounds 
        if args.len() < 3 {
            return Err("not enough arguments");
        }
        // saving the arg values in variables
        let query = args[1].clone(); // cloning allows us to not worry about lifetime references
        let filename = args[2].clone();

        Ok(Config { query, filename }) // returning the query and filename, yielding ownership
    }
}
