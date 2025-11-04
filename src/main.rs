use std::env;
use std::fs;
use std::process;
use std::error::Error;
use rav_grep::search;
fn main() {
let args: Vec<String> = env::args().collect();
let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
        
    });
        println!("Searching for {}", config.query);
        println!("In file {}", config.file_path);
        //Pattern matching to handle the Result returned by the run function.
        if let Err(e) = run(config) {
            println!("Application error: {e}");
            process::exit(1);
        }
        // run(config);

}

fn run(config: Config)-> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //Expect here fails the code for us if we cannot read the file or any other error occurs.
        for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}


struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str>  {
        if args.len() < 3 {

            //Panic stops the execution of the program immediately and displays the message passed to it.
            return Err("not enough arguments");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
//An expression returns the value, now the difference between statement and expression is that expression has no semicolon at the end while statement has a semicolon at the end.
// It also return a Config struct with the query and file_path fields set to the values passed as command line arguments.
//Statements return None while expressions return a value.




//This is a Rust program to replicate the Grep prograp.
//std::env::args is used to get the command line arguments passed to the program.