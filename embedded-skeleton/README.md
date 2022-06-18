# Rust AVR Skeleton
Some scripts and configuration files which is all you will need to
create, build, and flash an AVR device, like an Arduino Nano, with a program written
in Rust

**Note:** the `flash.sh` script uses avrdude directly to flash the board, and it was a good
exercise in bash scripting to write this, however upon reading the avr-hal repo
documentation, I see that a more feature rich version of this script exists in the form of 
the ravedude project. Take a look into that to easily integrate flashing the board with
your cargo workflow.
