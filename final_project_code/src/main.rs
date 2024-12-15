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

    // Read CSV
    processor.read_csv()?;

    // Clean CSV
    let cleaned_data = processor.clean_csv()?;

    // Write cleaned data to a new file
    let new_file_path = "influencers_september_cleaned.csv";
    processor.write_csv(cleaned_data)?;

    // Analyze CSV
    processor.analyze_csv()?;

    let df = CsvReader::from_path("influencers_september_cleaned.csv")?
        .has_header(true)    
        .finish()?;          

    // Print the DataFrame
    println!("{:?}", df);

    Ok(())
}