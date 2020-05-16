# minigrep
rust book I/O project follow along

## running
the program will check for the environment variable `CASE_SENSITIVE`.
If this environment variable exists, then the program will only print out strings that match exactly what you enter.
Otherwise it will print out anything that matches the entered string.

The format to run is `minigrep QUERY FILENAME` where `QUERY` is the query string to look for
and `FILENAME` is the file to look in it for. `FILENAME` follows POSIX dir structure so you can run this program using
absolute or relative path names.

## Example:

`CASE_SENSITIVE=1 ./minigrep Pick testfiles/test.txt` 

will search for `Pick` in the test.txt file in the testfiles directory
relative to the current working directory. Since the `CASE_SENSITIVE` environment variable was set, the query will be case
sensitive.
