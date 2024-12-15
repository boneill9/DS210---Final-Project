mod navigate_Data;
use crate::navigate_Data::CsvFileProcessor; // Ensure this is the correct import
use std::error::Error;
use polars::prelude::*;
use std::error::Error;


fn main() -> Result<(), Box<dyn Error>> {
    // Create a processor instance
    let processor = CsvFileProcessor;

    // Specify the input and output file paths
    let input_path = "data/influencers_september.csv";  
    let output_path = "influencers_september_cleaned.csv"; 

    // Call the clean_csv method
    processor.clean_csv(input_path, output_path)?;

    println!("CSV processing complete. Output saved to {}", output_path);

    Ok(())
}
