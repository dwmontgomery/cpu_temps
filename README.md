# CS417 Semester Project
Author: Dylan Montgomery

## Requirements

This program is written in Rust using Cargo for build management. 

    * Rust 1.84.0

## Purpose

The purpose of this program is to analyze the temperatures of a four-core CPU running over a period of time. 

## Input Format

Data input takes the form of temperatures in a txt file. All data points are whitespace delimited. The input can be expected one of two ways:

with labels, such as,

    +61.0°C +63.0°C +50.0°C +58.0°C
    +80.0°C +81.0°C +68.0°C +77.0°C
    +62.0°C +63.0°C +52.0°C +60.0°C
    +83.0°C +82.0°C +70.0°C +79.0°C
    +68.0°C +69.0°C +58.0°C +65.0°C

or without lables, such as, 

    61.0 63.0 50.0 58.0
    80.0 81.0 68.0 77.0
    62.0 63.0 52.0 60.0
    83.0 82.0 70.0 79.0
    68.0 69.0 58.0 65.0

Each line represents temperature readings from 4 processor cores. Readings are taken every 30 seconds. In this example:

> line 1 is 0 sec  
> line 2 is 30 sec,  
> line 3 is 60 sec.  
> line 4 is 90 sec.  
> line 5 is 120 sec.  


## Output Format

All output must be written to text files (one file per core). Each line must take the form:

> *x<sub>k</sub>* <=    *x* < *x<sub>k+1</sub>* ; *y<sub>i</sub>* = *c<sub>0</sub>*+*c<sub>1</sub>x* ; _type_

where
   - *x<sub>k</sub>* and *x<sub>k+1</sub>* are the domain in which *y<sub>k</sub>* is applicable
   - *y<sub>k</sub>* is the k<sup>th</sup> function
   - *type* is either *least-squares* or *interpolation*

For the example data in described in Section 2.1 (Input Format) you would generate 4 output files.

   - {basename}-core-0.{txt}
   - {basename}-core-1.{txt}
   - {basename}-core-2.{txt}
   - {basename}-core-3.{txt}

where {basename} is the input file name without the extension (e.g., without the .txt or .dat).


## Sample Execution and Output

### Sample Execution

#### Building the program 

The program can be built using Cargo by running the following command

    cargo build

#### Running the program

The program can be run using the command

    ./target/release/cpu_temps "./input/file/path.txt"

or can be run using Cargo. To run using Cargo, use the command

    cargo run -- "./input/file/path.txt"

If run without a command line argument, such as 

    ./target/release/cpu_temps

the following will be displayed

    invalid command line argument: expected ./target/releaase/cpu_temps <input_file_path>

A similar error will also be displayed if any input file other than a .txt file is used. 

    invalid input file type: expected <file>.txt


### Sample Output

If run using a command line argument, such as 

    ./target/release/cpu_temps "sample_input.txt"

with a sample input file similar to the following, where each column 
represents a single core of a CPU:

    61.0 63.0 50.0 58.0
    80.0 81.0 68.0 77.0
    62.0 63.0 52.0 60.0
    83.0 82.0 70.0 79.0
    68.0 69.0 58.0 65.0

The output file should be similar to the following:

    0 <= x <=       30 ; y =        61.0000 +       0.6333 x ; interpolation
    30 <= x <=      60 ; y =        98.0000 +       -0.6000 x ; interpolation
    60 <= x <=      90 ; y =        20.0000 +       0.7000 x ; interpolation
    90 <= x <=      120 ; y =       128.0000 +      -0.5000 x ; interpolation
    0 <= x <=       120 ; y =       67.4000 +       0.0567 x ; least-squares

Note that the above sample output file is for a single core. There will be three similar 
outputs files for the other remaining cores. 

Output files are saved to the /output directory. If the directory does not exist, then it is created. 
