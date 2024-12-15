mod k_means;
use k_means::{KMeans, plot_data};
mod navigate_Data;
use crate::navigate_Data::CsvFileProcessor; // Ensure this is the correct import
use std::error::Error;
use polars::prelude::*;
use ndarray::Array2;


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

    println!("{:?}", df.dtypes());


    let col1: Vec<f64> = df["Followers"].f64()?
    .into_iter()
    .filter_map(|x| x) // Removes `None` values, extracting the `Some(f64)` values
    .collect();
    let col2: Vec<f64> = df["Likes (Avg.)"].f64()?
    .into_iter()
    .filter_map(|x| x) // Removes `None` values, extracting the `Some(f64)` values
    .collect();
    let data = Array2::from_shape_vec((col1.len(), 2), col1.into_iter().chain(col2.into_iter()).collect())?;

    // Perform K-means with 3 clusters and 100 iterations using the trait method
    let k = 3;
    let labels = data.kmeans(k, 100); // This uses the kmeans method from the KMeans trait

    // Plot the results
    plot_data(&data, &labels, k)?;


    Ok(())
}
