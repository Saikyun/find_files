# find_files
Very simple file finder using glob.

## usage
find_files <subdir> "<glob-pattern>"

e.g. find_files /var/www "*.php"
 -- finds all php-files in /var/www

e.g. find_files /var/www "**/*.php"
 -- finds all php-files in /var/www and its subdirectories

read more here: https://doc.rust-lang.org/glob/glob/struct.Pattern.html
