use csv::{ReaderBuilder, WriterBuilder};
use regex::Regex;
use serde::Deserialize;
use std::error::Error;
use polars::prelude::*;
use serde::ser::StdError;

// Struct to represent a CSV File Processor
pub struct CsvFileProcessor {
    pub file_path: String,
}

#[derive(Debug, Deserialize)]
pub struct Record {
    field1: String,
    field2: Option<i32>, // Example of handling missing numeric values
}

// Trait for processing CSV files
pub trait CsvProcessor {
    fn read_csv(&self) -> Result<(), Box<dyn Error>>;
    fn clean_csv(&self, output_file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>>;
    fn analyze_csv(&self) -> Result<(), Box<dyn Error>>;
    fn write_csv(
        &self,
        headers: &[String],         // Pass headers as a slice of strings
        records: Vec<Vec<String>>,  // The cleaned data
        output_file_path: &str, 
    ) -> Result<(), Box<dyn Error>>;
}

// Implementing the CsvProcessor trait for CsvFileProcessor
impl CsvProcessor for CsvFileProcessor {
    // Step 1: Read CSV file
    fn read_csv(&self) -> Result<(), Box<dyn Error>> {
        let mut reader = ReaderBuilder::new().from_path(&self.file_path)?;

        println!("Reading CSV file:");
        for result in reader.records() {
            let record = result?;
            println!("{:?}", record);
        }

        Ok(())
    }

    fn clean_csv(&self, data: Vec<String>) -> Vec<String> {
        let re = Regex::new(r"(?i)(\d+(?:\.\d+)?)([mk])?").unwrap();
    
        // Iterate over each record in the data vector
        let cleaned_record: Vec<String> = data.iter()
            .map(|field| {
                if let Some(caps) = re.captures(field) {
                    let value = caps[1].parse::<f64>().unwrap_or(0.0);
                    let multiplier = match caps.get(2).map(|m| m.as_str()) {
                        Some("M") | Some("m") => 1_000_000.0,
                        Some("K") | Some("k") => 1_000.0,
                        _ => 1.0,
                    };
                    (value * multiplier).to_string()
                } else {
                    field.to_string()
                }
            })
            .collect();
    
        cleaned_record
    }
    
    
    

    // Step 3: Analyze CSV file
    fn analyze_csv(&self) -> Result<(), Box<dyn Error>> {
        println!("Analyzing CSV file:");

        // Using Polars for advanced analysis
        let df = CsvReader::from_path(&self.file_path)?
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

        // Basic Aggregation Example
        let mut reader = ReaderBuilder::new().from_path(&self.file_path)?;
        let mut total = 0;
        let mut count = 0;

        for result in reader.records() {
            let record = result?;
            if let Some(value) = record.get(1) { // Assume column 1 has numeric data
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
    fn write_csv(
        &self,
        headers: &[String],         // Pass headers as a slice of strings
        records: Vec<Vec<String>>,  // The cleaned data
        output_file_path: &str,     // Path for the output CSV file
    ) -> Result<(), Box<dyn Error>> {
        let mut writer = csv::Writer::from_path(output_file_path)?;
    
        // Write the headers
        writer.write_record(headers)?;
    
        // Write the records
        for record in records {
            writer.write_record(record)?;
        }
    
        writer.flush()?; // Ensure everything is written to the file
    
        Ok(())
    }
    
    
    
    
    
}
