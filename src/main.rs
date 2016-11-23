extern crate glob;
use std::env;
use glob::glob;

fn main() {
    let mut path: String = match env::args().nth(1) {
        Some(ref str) if str == "--help" || str == "-h" || str == "help" => {
            println!("usage: find_files <subdir> \"<glob-pattern>\"");
            println!("");
            println!("e.g. find_files /var/www \"*.php\"");
            println!(" -- finds all php-files in /var/www");
            println!("");
            println!("e.g. find_files /var/www \"**/*.php\"");
            println!(" -- finds all php-files in /var/www and its subdirectories");
            println!("");
            println!("read more here: https://doc.rust-lang.org/glob/glob/struct.Pattern.html");
            return;
        },
        Some(path) => path,
        None => { println!("error: first argument is path in which files reside.");
                  println!("run find_files help for more information.");
                  return; }
    };
        
    path = match path.pop() {
        Some('/') => path + "/",
        Some(c) => path + &c.to_string() + "/",
        None => { println!("error: path is zero characters long.");
                  println!("\"find_files help\" for more information.");
                  return; },
    };

    let pattern: String = if let Some(pattern) = env::args().nth(2) {
        pattern
    } else {
        println!("error: second argument is pattern to find files.");
        println!("\"find_files help\" for more information.");
        return;
    };

    let string_and_pattern = path + &pattern;

    let globber = glob(&string_and_pattern).expect("Derpi");

    for entry in globber {
        match entry {
            Ok(path) => println!("{:?}", path.display()),
            Err(e) => println!("{:?}", e),
        }
    }
}
