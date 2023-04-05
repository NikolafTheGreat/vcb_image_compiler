# vcb_image_compiler

## Intro

Just a quick and dirty program that takes a 64x64 pixel image, a text file with a palette and spits out a text file containing the viirtual memory needed for my virtual circuit board display.

## Compiling

I used no special compiler flags when i compile it. Just use cargo and you should be fine.
If you don't know how to compile a rust project se this: https://doc.rust-lang.org/cargo/guide/

## Running

This is a command line tool that expects it's arguments in the following order:
  * Path to the input image
  * Path to the colour pallet file
  * Path to the output file (will be created if it doesn't already exist)

## Color palette file

The color palette file should be a list of hex colors on seperate lines with a "#" character preceding the hexadecimal digits (Capital letters). If you put in more than 16 the program will throw an error.

## Support

I am not planning on supporting this. I just made this to share a virtual circuit board project that i made.
