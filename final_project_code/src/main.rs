mod graphs;
mod csv_reading;
use crate::csv_reading::{CsvProcessor, CsvHandler};
use polars::prelude::CsvReader;

use graphs::{read_tiktokers_from_csv, build_similarity_graph, visualize_graph};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let handler = CsvHandler;

    let file_path = "data/influencers_september.csv";
    let cleaned_file_path = "cleaned_output_sept.csv";

    // CSV Processing
    handler.read_csv(file_path)?;
    let (cleaned_data, headers) = handler.clean_csv(file_path)?;
    handler.write_cleaned_csv(cleaned_file_path, cleaned_data, headers)?;
    handler.analyze_csv(cleaned_file_path)?;

}


