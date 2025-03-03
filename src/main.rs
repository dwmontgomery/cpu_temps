pub mod piecewise_linear;

use std::env;
use std::process;
use temperature_parser::read_temperature_file;
use piecewise_linear::Points;

fn main() {
    
    // Read the arguments from the command line
    let args: Vec<String> = env::args().collect();

    // If no command line arguments are given, display and error and exit
    if args.len() != 2 {
        eprintln!("Invalid command line argument: expected \" {} <file name> \" ", args[0]);
        process::exit(1);
    }

    // If input file is not a .txt, give an error and exit
    if !args[1].ends_with(".txt") {
        eprintln!("invalid input file type: expected <file>.txt");
        process::exit(1);
    }

    let temp_data = match read_temperature_file(&args[1]) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error reading temperature file: {}", e);
            process::exit(1);
        }
    };

    // Initialize a vector for each core and for the times     
    let mut times: Vec<f64> = vec![];
    let mut readings_core_0: Vec<f64> = vec![];
    let mut readings_core_1: Vec<f64> = vec![];
    let mut readings_core_2: Vec<f64> = vec![];
    let mut readings_core_3: Vec<f64> = vec![];

    for line in temp_data {
        times.push(line.time_step as f64);
        readings_core_0.push(line.readings[0]);
        readings_core_1.push(line.readings[1]);
        readings_core_2.push(line.readings[2]);
        readings_core_3.push(line.readings[3]);
    }


    // Create Points structs for each core
    let core_0_points = Points::new(times.clone(), readings_core_0);
    let core_1_points = Points::new(times.clone(), readings_core_1);
    let core_2_points = Points::new(times.clone(), readings_core_2);
    let core_3_points = Points::new(times.clone(), readings_core_3);

    // Interpolate the data for each core
    let mut core_0_points = core_0_points;
    let mut core_1_points = core_1_points;
    let mut core_2_points = core_2_points;
    let mut core_3_points = core_3_points;

    core_0_points.interpolate();
    core_1_points.interpolate();
    core_2_points.interpolate();
    core_3_points.interpolate();

    // Optionally print out the interpolated values for each core
    println!("Core0");
    core_0_points.print();

    println!("Core1");
    core_1_points.print();

    println!("Core2");
    core_2_points.print();

    println!("Core3");
    core_3_points.print();
}