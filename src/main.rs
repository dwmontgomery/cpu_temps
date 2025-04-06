pub mod temperature_analyzer;
pub mod output;

use std::env;
use std::process;
use std::path::Path;
use temperature_parser::read_temperature_file;
use temperature_analyzer::Points;
use output::write_temps;


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

    // Read the data from the input file
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

    // Push the data from the read data to the respective core vectors
    for line in temp_data {
        times.push(line.time_step as f64);
        readings_core_0.push(line.readings[0]);
        readings_core_1.push(line.readings[1]);
        readings_core_2.push(line.readings[2]);
        readings_core_3.push(line.readings[3]);
    }


    // Create Points structs for each core using the input data for the core and 
    // the time vector
    let mut core_0_points = Points::new(times.clone(), readings_core_0);
    let mut core_1_points = Points::new(times.clone(), readings_core_1);
    let mut core_2_points = Points::new(times.clone(), readings_core_2);
    let mut core_3_points = Points::new(times.clone(), readings_core_3);

    // Calculate the values for the linear interpolation
    core_0_points.interpolate();
    core_1_points.interpolate();
    core_2_points.interpolate();
    core_3_points.interpolate();

    // Calculate the values for the least squares approximation
    core_0_points.least_squares();
    core_1_points.least_squares();
    core_2_points.least_squares();
    core_3_points.least_squares();

    // Check if the output directory exists. If not, create it
    let output_dir = "./output";
    if !Path::new(output_dir).exists() {
        std::fs::create_dir_all(output_dir).expect("Failed to create output directory");
    }

    // Retreive the base file name from the input file and format the file path
    let basename = Path::new(&args[1])
        .file_stem() 
        .and_then(|s| s.to_str())
        .unwrap_or("");

    let filename0 = format!("./output/{}-core-0.txt", basename);
    let filename1 = format!("./output/{}-core-1.txt", basename);
    let filename2 = format!("./output/{}-core-2.txt", basename);
    let filename3 = format!("./output/{}-core-3.txt", basename);


    // Write the data to the output files in the output directory
    if let Err(e) = write_temps(&core_0_points, &filename0) {
        eprintln!("Error writing file: {}", e);
    }

    if let Err(e) = write_temps(&core_1_points, &filename1) {
        eprintln!("Error writing file: {}", e);
    }    

    if let Err(e) = write_temps(&core_2_points, &filename2) {
        eprintln!("Error writing file: {}", e);
    }

    if let Err(e) = write_temps(&core_3_points, &filename3) {
        eprintln!("Error writing file: {}", e);
    }
}