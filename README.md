# find_files
Very simple file finder using glob.

## install
1. clone the project
2. `cargo build --release`
3. copy the target/release/find_files_by_name binary to wherever you want it
4. use it

## usage
`find_files <subdir> "<glob-pattern>"`

e.g. `find_files /var/www "*.php"`<br>
 -- finds all php-files in /var/www

e.g. `find_files /var/www "**/*.php"`<br>
 -- finds all php-files in /var/www and its subdirectories

read more here: https://doc.rust-lang.org/glob/glob/struct.Pattern.html
