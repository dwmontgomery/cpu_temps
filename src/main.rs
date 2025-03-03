use std::env;
use std::process;

fn main() {
    
    // Read the arguments from the command line
    let args: Vec<String> = env::args().collect();

    // If no command line arguments are given, display and error and exit
    if args.len() != 2 {
        eprintln!("Invalid command line argument: expected \" {} <file name>" \"", args[0]);
        process::exit(1);
    }

    // If input file is not a .txt, give an error and exit
    if args[1].ends_with(".txt") {
        eprintln!("invalid input file type: expected <file>.txt");
        process::exit(1);
    }

    // Initialize a vector for each core and for the times     
    let mut times: Vec<f64> = Vec![];
    let mut readings_core_0: Vec<f64> = Vec![];
    let mut readings_core_1: Vec<f64> = Vec![];
    let mut readings_core_2: Vec<f64> = Vec![];
    let mut readings_core_3: Vec<f64> = Vec![];

    read_temperature_file(args[1]);


}