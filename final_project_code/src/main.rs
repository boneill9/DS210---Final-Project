mod k_means;
mod navigate_Data;
use k_means::{KMeans, plot_data};
use crate::navigate_Data::CsvFileProcessor; // Ensure this is the correct import
use std::error::Error;
use polars::prelude::*;
use ndarray::Array2;
use polars::prelude::DataType::Float64;

use polars::prelude::*;

fn extract_floats_from_column(
    df: &DataFrame,
    column_name: &str
) -> Result<Vec<f64>, PolarsError> {
    // Get the column as a Series
    let series = df.column(column_name)?;

    // Cast the column to Float64 if necessary
    let float_series = series.cast(&DataType::Float64)?;

    // Convert the Series to Vec<f64>
    let float_values: Vec<f64> = float_series.f64()?.into_no_null_iter().collect();

    // Perform normalization (min-max scaling)
    let min = float_values.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = float_values.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let normalized_values: Vec<f64> = if (max - min).abs() > f64::EPSILON {
        // Avoid division by zero if max == min
        float_values.iter().map(|&x| (x - min) / (max - min)).collect()
    } else {
        // If all values are the same, return a vector of zeros
        vec![0.0; float_values.len()]
    };

    Ok(normalized_values)
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

    //let likes_vs_followers: Vec<Vec<f64>> = likes
    //.iter()
    //.zip(followers.iter())
    //.map(|(&like, &follower)| vec![like, follower])
    //.collect();

    let followers_vs_likes: Vec<Vec<f64>> = followers
    .iter()
    .zip(likes.iter())
    .map(|(&follower, &like)| vec![follower, like])  
    .collect();


    let k = 3;
    let max_iters = 100;

    //let labels = likes_vs_followers.kmeans(k, max_iters);
    let labels = followers_vs_likes.kmeans(k, max_iters);

    // Print cluster assignments
    //println!("Cluster Assignments: {:?}", labels);

    // Plot the data with cluster labels
    if let Err(e) = plot_data(&followers_vs_likes, &labels, k, "followers_vs_likes.png") {
        eprintln!("Error generating plot: {}", e);
    }

    let shares_vs_likes: Vec<Vec<f64>> = shares
    .iter()
    .zip(likes.iter())
    .map(|(&share, &like)| vec![share, like])  
    .collect();


    //let labels = likes_vs_followers.kmeans(k, max_iters);
    let labels = shares_vs_likes.kmeans(k, max_iters);

    // Print cluster assignments
    //println!("Cluster Assignments: {:?}", labels);

    // Plot the data with cluster labels
    if let Err(e) = plot_data(&shares_vs_likes, &labels, k, "shares_vs_likes.png") {
        eprintln!("Error generating plot: {}", e);
    }

    let followers_vs_shares: Vec<Vec<f64>> = followers
    .iter()
    .zip(shares.iter())
    .map(|(&follower, &share)| vec![follower, share])  
    .collect();


    //let labels = likes_vs_followers.kmeans(k, max_iters);
    let labels = followers_vs_shares.kmeans(k, max_iters);

    // Print cluster assignments
    //println!("Cluster Assignments: {:?}", labels);

    // Plot the data with cluster labels
    if let Err(e) = plot_data(&followers_vs_shares, &labels, k, "followers_vs_shares.png") {
        eprintln!("Error generating plot: {}", e);
    }

    let comments_vs_likes: Vec<Vec<f64>> = comments
    .iter()
    .zip(likes.iter())
    .map(|(&comment, &like)| vec![comment, like])  
    .collect();


    //let labels = likes_vs_followers.kmeans(k, max_iters);
    let labels = comments_vs_likes.kmeans(k, max_iters);

    // Print cluster assignments
    //println!("Cluster Assignments: {:?}", labels);

    // Plot the data with cluster labels
    if let Err(e) = plot_data(&comments_vs_likes, &labels, k, "comments_vs_likes.png") {
        eprintln!("Error generating plot: {}", e);
    }

    let followers_vs_views: Vec<Vec<f64>> = followers
    .iter()
    .zip(views.iter())
    .map(|(&follower, &like)| vec![follower, like])  
    .collect();


    //let labels = likes_vs_followers.kmeans(k, max_iters);
    let labels = followers_vs_views.kmeans(k, max_iters);

    // Print cluster assignments
    //println!("Cluster Assignments: {:?}", labels);

    // Plot the data with cluster labels
    if let Err(e) = plot_data(&followers_vs_views, &labels, k, "followers_vs_views.png") {
        eprintln!("Error generating plot: {}", e);
    }

    Ok(())

    
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_plot_data() {
        let data = vec![
            vec![1.0, 2.0],
            vec![2.0, 3.0],
            vec![3.0, 4.0],
        ];
        let labels = vec![0, 1, 2];
        let k = 3;
        let filename = "test_plot.png";

        // Ensure file doesn't exist before plotting
        if fs::metadata(filename).is_ok() {
            fs::remove_file(filename).unwrap();
        }

        // Run the plotting function
        let result = plot_data(&data, &labels, k, filename);

        // Check if the result is Ok and file is created
        assert!(result.is_ok());
        assert!(fs::metadata(filename).is_ok());

        // Clean up by removing the plot file after test
        fs::remove_file(filename).unwrap();
    }

}

