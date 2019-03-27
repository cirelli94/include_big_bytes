## Why
This is a little project to use a bunch of bytes concatenated to the executable, because using [`include_bytes`](https://doc.rust-lang.org/std/macro.include_bytes.html) failed to compile with a file of ~1GB on my machine.

## How-to
* Compile
* Get program and data size with `stat` in bytes: `stat -c "%s %n" file`
* Concatenate your data to the program: `cat data >> program`
* Add this info to the bottom of your program, that will use it: `echo "FOOTER;program_size;data_size" >> program`
