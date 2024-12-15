mod k_means;
mod navigate_Data;
use k_means::{KMeans, plot_data};
use crate::navigate_Data::CsvFileProcessor; // Ensure this is the correct import
use std::error::Error;
use polars::prelude::*;
use ndarray::Array2;
use polars::prelude::DataType::Float64;

use polars::prelude::*;

fn extract_floats_from_column(df: &DataFrame, column_name: &str) -> Result<Vec<f64>, PolarsError> {
    // Get the column as a Series
    let series = df.column(column_name)?;

    // Cast the column to Float64 if necessary
    let float_series = series.cast(&DataType::Float64)?;

    // Convert the Series to Vec<f64>
    let float_values: Vec<f64> = float_series.f64()?.into_no_null_iter().collect();

    Ok(float_values)
}


fn main() -> Result<(), Box<dyn Error>> {
    // Create a processor instance
    let processor = CsvFileProcessor;

    // Specify the input and output file paths
    let input_path = "data/influencers_september.csv";  
    let output_path = "influencers_september_cleaned.csv"; 

    // Call the clean_csv method
    processor.clean_csv(input_path, output_path)?;

    println!("CSV processing complete. Output saved to {}", output_path);

    let df = CsvReader::from_path(output_path)?
        .has_header(true)
        .finish()?;

    println!("Original DataFrame:\n{}", df);

    let mut df = df.clone();

    let views: Vec<f64> = extract_floats_from_column(&mut df, "Views (Avg.)")?;
    let likes: Vec<f64> = extract_floats_from_column(&mut df, "Likes (Avg.)")?;
    let comments: Vec<f64> = extract_floats_from_column(&mut df, "Comments (Avg.)")?;
    let shares: Vec<f64> = extract_floats_from_column(&mut df, "Shares (Avg.)")?;
    let followers: Vec<f64> = extract_floats_from_column(&mut df, "Followers")?;

    let likes_vs_followers: Vec<Vec<f64>> = vec![likes, followers];
    let k = 3;
    let max_iters = 100;

    let labels = likes_vs_followers.kmeans(k, max_iters);

    // Print cluster assignments
    println!("Cluster Assignments: {:?}", labels);

    // Plot the data with cluster labels
    if let Err(e) = plot_data(&likes_vs_followers, &labels, k) {
        eprintln!("Error generating plot: {}", e);
    }

    Ok(())

    
}