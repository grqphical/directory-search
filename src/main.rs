use std::env;
use regex::Regex;
use std::fs;

fn main() {
    // Get CLI args
    let args: Vec<String> = env::args().collect();
    
    // Get the regex pattern
    let pattern = Regex::new(&args[1]).unwrap();

    // Get a list of all files in our current directory
    let paths = fs::read_dir("./").unwrap();

    // loop through each and file and check if the pattern matches, if it does print it
    for path in paths {
        if pattern.is_match(path.as_ref().unwrap().path().to_str().unwrap())
        {
            println!("{}", path.unwrap().path().display());
        }
    }
}
