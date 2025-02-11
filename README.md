# CS417 Semester Project
Author: Dylan Montgomery

## Requirements

This program is written in Rust using Cargo for build management. 

    * Rust 1.84.0

## Purpose

## Sample Execution and Output

### Sample Execution

The program can be run using the command

    <to be added>

If run without a command line argument, such as 

    <to be added>

the following will be displayed

    <to be added>

### Sample Output

If run using a command line argument, such as 

    <to be added> <filename.txt>

with the file-line sample input

    61.0 63.0 50.0 58.0
    80.0 81.0 68.0 77.0
    62.0 63.0 52.0 60.0
    83.0 82.0 70.0 79.0
    68.0 69.0 58.0 65.0

where each column represent the temperatures for a single core of a CPU. 

Then the output file should be similar to the following:

    0 <= x <=       30 ; y =        61.0000 +       0.6333 x ; interpolation
    30 <= x <=      60 ; y =        98.0000 +       -0.6000 x ; interpolation
    60 <= x <=      90 ; y =        20.0000 +       0.7000 x ; interpolation
    90 <= x <=      120 ; y =       128.0000 +      -0.5000 x ; interpolation
    0 <= x <=       120 ; y =       67.4000 +       0.0567 x ; least-squares

Note that the above sample output is for a single core. There will be three other outputs files
for the other remaining cores. 
