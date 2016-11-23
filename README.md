# find_files
Very simple file finder using glob.

## install
clone the project
`cargo build --release`
copy the target/release/find_files_by_name binary to wherever you want it
use it

## usage
`find_files <subdir> "<glob-pattern>"`

e.g. `find_files /var/www "*.php"`
 -- finds all php-files in /var/www

e.g. `find_files /var/www "**/*.php"`
 -- finds all php-files in /var/www and its subdirectories

read more here: https://doc.rust-lang.org/glob/glob/struct.Pattern.html
