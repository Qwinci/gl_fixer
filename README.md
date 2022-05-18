# gl_fixer
A tool to make inline gl functions from glad headers that just call glad functions. Intended to be used with buggy editors which don't display argument names for function pointers.

### Usage (build from source)
- Copy your gl.h file into the root directory of this thing.
- Run `cargo run` in the root directory of this thing in a shell.
- The tool generated file `gl_fixed.h`, which you can include in your project (you still need the original glad c file and headers).
### Usage (binary)
- Download the latest release from the releases page (`gl_fixer` for Linux and `gl_fixer.exe` for Windows).
- Put glad `gl.h` into same directory as the downloaded executable and run the executable.
- The tool generated file `gl_fixed.h`, which you can include in your project (you still need the original glad c file and headers). 
