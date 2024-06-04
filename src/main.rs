use gds21::GdsLibrary;
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::fs::File;
use std::io::Write;

use std::env;
use std::path::Path;


fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();
    
    // Check if the correct number of arguments is provided
    if args.len() != 2 {
        eprintln!("Usage: {} <input_gds_file>", args[0]);
        std::process::exit(1);
    }


    // Get the input GDS file path from the arguments
    let gds_file_path = &args[1];

    // Create the output JSON file path based on the input file name
    let input_path = Path::new(gds_file_path);
    let output_file_name = input_path.file_stem().unwrap().to_str().unwrap();
    let json_file_path = format!("{}.json", output_file_name);


    // Load the GDS library
    let lib: GdsLibrary = GdsLibrary::load(gds_file_path).expect("Failed to load GDS file");

    // Serialize the GDS library to JSON format
    let json = to_string_pretty(&lib).expect("Failed to serialize GDS library to JSON");

    // Write the JSON to a file
    let mut file = File::create(&json_file_path).expect("Failed to create JSON file");
    file.write_all(json.as_bytes()).expect("Failed to write JSON to file");

    println!("GDS library has been successfully written to {}", json_file_path);
}
