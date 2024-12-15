mod navigate_Data;
use polars::prelude::CsvReader;
use std::error::Error;
use std::fmt;
use polars::frame::DataFrame;
use polars::prelude::SerReader;
use crate::navigate_Data::CsvFileProcessor;

// Import the CsvProcessor trait to use its methods
use crate::navigate_Data::CsvProcessor; // <-- Ensure this import is here

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "data/influencers_september.csv"; // Input CSV file path
    let processor = CsvFileProcessor { file_path: file_path.to_string() };

    // Step 1: Read CSV
    processor.read_csv()?;

    // Step 2: Clean CSV
    let output_file_path = "influencers_september_cleaned.csv";
    let cleaned_data = processor.clean_csv(output_file_path)?;
    println!("Cleaned Data: {:?}", cleaned_data);


    // Step 4: Verify the cleaned file exists
    if !std::path::Path::new(output_file_path).exists() {
        println!("Error: Cleaned file does not exist!");
        return Ok(());
    }

    // Step 5: Analyze the cleaned CSV file using Polars
    let df = CsvReader::from_path(output_file_path)?
        .has_header(true)
        .finish()?;
    println!("DataFrame loaded from cleaned CSV:\n{:?}", df);

    Ok(())
}

