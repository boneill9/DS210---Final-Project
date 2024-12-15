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
    println!("Cleaned data: {:?}", cleaned_data);

    // Write cleaned data to a new file
    let new_file_path = "influencers_september_cleaned.csv";
    processor.write_csv(cleaned_data, &[0, 2, 3, 4, 5, 6, 7])?;
    println!("Cleaned CSV file written to: {}", new_file_path);

    // Verify the cleaned file exists
    if !std::path::Path::new(new_file_path).exists() {
        println!("Error: Cleaned file does not exist!");
        return Ok(());
    }

    // Load cleaned data into DataFrame
    let df = CsvReader::from_path(file_path)?
        .has_header(true)
        .finish()?;

    // Print the DataFrame
    println!("DataFrame loaded from cleaned CSV:\n{:?}", df);

    Ok(())
}
