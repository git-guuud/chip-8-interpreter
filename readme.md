# CHIP-8 Interpreter

A simple CHIP-8 interpreter written in Rust. The "Hello, World!" of emulator developement.
### What is CHIP-8? 
CHIP-8 is an interpreted programming language that was created in the 1970s for use on early microcomputers. It is a simple, stack-based language that is designed to be easy to learn and use. <br>


# How to use?
```bash
./chip-8.exe /path/to/rom

Options:
  -m, --move-val-8xy6e
  -b, --bxnn
  -i, --increment-i-on-load
  -h, --help                 Print help
```

Some flags are there to accomodate some of the different implementations of CHIP-8.

**--move-val-8xy6e:** OFF by default include flag to turn ON<br> 
Sets whether the value in register Vx is moved to the I register when executing the instructions 8XY6 and 8XYE. 

**--bxnn:** OFF by default include flag to turn ON<br>
If ON pc is set to XNN + VX, if OFF pc is set to XNN + V0.

**--increment-i-on-load:** OFF by default include flag to turn ON<br>
Whether the I register gets incremented on loading registers from memory on instructions FX55 and FX65.

### Where can i get the ROMs?
Download some .ch8 files from [here.](https://github.com/kripod/chip8-roms/tree/master/games)
