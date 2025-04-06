use std::fs::File;
use std::io::{self, Write};
use crate::temperature_analyzer;
use temperature_analyzer::Points;

// File to handle the printing and output for the cpu_temps program

pub fn print(points: Points) {
    for k in 0..points.x_values.len()-1 {
        println!("{:8} <= x < {:8} ; y = {:16.04} + {:8.04} x ; interpolation", points.x_values[k], points.x_values[k+1], points.b_values[k], points.m_values[k]);
    }
    
}

pub fn write_temps(points: &Points, file_name: &str) -> io::Result<()> {
    // Create or overwrite the file
    let mut file = File::create(file_name)?;

    let len = points.x_values.len()-1;
    println!("Writing to file {}", file_name);
    for k in 0..len {
        writeln!(
            file,
            "{:8} <= x < {:8} ; y = {:10.04} + {:10.04} x ; interpolation",
            points.x_values[k],
            points.x_values[k + 1],
            points.b_values[k],
            points.m_values[k]
        )?;
    }
    writeln!(file,
        "{:8} <= x < {:8} ; y = {:10.04} + {:10.04} x ; least-squares",
        0,
        points.x_values[len],
        points.least_sq_b,
        points.least_sq_m
        )?;
    println!("complete");
    Ok(())
}