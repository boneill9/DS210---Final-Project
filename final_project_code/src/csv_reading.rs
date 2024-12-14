use csv::{ReaderBuilder, WriterBuilder}; // Import necessary structs from the csv crate
use polars::prelude::CsvReader; // Import CsvReader from polars
use std::fs::File;
use crate::Error;



pub trait CsvProcessor {
    fn read_csv(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>>;
    fn clean_csv(&self, file_path: &str) -> Result<(Vec<Vec<f64>>, csv::StringRecord), Box<dyn std::error::Error>>;
    fn write_cleaned_csv(&self, file_path: &str, cleaned_data: Vec<Vec<f64>>, headers: csv::StringRecord) -> Result<(), Box<dyn std::error::Error>>;
    fn analyze_csv(&self, file_path: &str) -> Result<(), Box<dyn std::error::Error>>;
}

pub struct CsvHandler;
    impl CsvProcessor for CsvHandler {
        // Implement the method to read CSV file and print records
        fn read_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
            let mut reader = ReaderBuilder::new().from_path(file_path)?;
    
            println!("Reading CSV file:");
            for result in reader.records() {
                let record = result?;
                println!("{:?}", record);
            }
    
            Ok(())
        }
    
        // Implement the method to clean the CSV data
        fn clean_csv(&self, file_path: &str) -> Result<(Vec<Vec<f64>>, csv::StringRecord), Box<dyn Error>> {
            let mut reader = ReaderBuilder::new().has_headers(true).from_path(file_path)?;
            let headers = reader.headers()?.clone();  // Clone the headers to keep them
            let mut cleaned_data = Vec::new();
    
            for result in reader.records() {
                let record = result?;
                let cleaned_record: Vec<f64> = record
                    .iter()
                    .map(|field| {
                        let input = field.trim();
                        if input.ends_with("M") {
                            let num = &input[..input.len() - 1];
                            let value: f64 = num.parse().unwrap_or(0.0);
                            value * 1_000_000.0
                        } else if input.ends_with("K") {
                            let num = &input[..input.len() - 1];
                            let value: f64 = num.parse().unwrap_or(0.0);
                            value * 1_000.0
                        } else {
                            input.parse().unwrap_or(0.0)
                        }
                    })
                    .collect();
                cleaned_data.push(cleaned_record);
            }
    
            Ok((cleaned_data, headers))
        }
    
        // Implement the method to write cleaned CSV data to a new file
        fn write_cleaned_csv(&self, file_path: &str, cleaned_data: Vec<Vec<f64>>, headers: csv::StringRecord) -> Result<(), Box<dyn Error>> {
            let mut writer = WriterBuilder::new().from_path(file_path)?;
            writer.write_record(&headers)?;
    
            for record in cleaned_data {
                writer.write_record(record.iter().map(|x| x.to_string()))?;
            }
    
            writer.flush()?;
            Ok(())
        }
    
        // Implement the method to analyze CSV file with Polars
        fn analyze_csv(&self, file_path: &str) -> Result<(), Box<dyn Error>> {
            println!("Analyzing CSV file:");
    
            let df = CsvReader::from_path(file_path)?
                .infer_schema(None)
                .has_header(true)
                .finish()?;
    
            println!("DataFrame Summary:");
            for column in df.get_columns() {
                let name = column.name();
                let mean = column.mean().unwrap_or(f64::NAN);
                let median = column.median().unwrap_or(f64::NAN);
                let count = column.len();
    
                println!(
                    "Column: {}, Mean: {:.2}, Median: {:.2}, Count: {}",
                    name, mean, median, count
                );
            }
    
            let mut reader = ReaderBuilder::new().from_path(file_path)?;
            let mut total = 0;
            let mut count = 0;
    
            for result in reader.records() {
                let record = result?;
                if let Some(value) = record.get(1) {
                    if let Ok(num) = value.parse::<i32>() {
                        total += num;
                        count += 1;
                    }
                }
            }
    
            if count > 0 {
                println!("Average of column 1: {}", total as f32 / count as f32);
            } else {
                println!("No numeric data to analyze in column 1.");
            }
    
            Ok(())
        }
    }

