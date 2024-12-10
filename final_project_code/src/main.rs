use csv::{ReaderBuilder, WriterBuilder};
use polars::prelude::*;
use regex::Regex;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct Record {
    field1: String,
    field2: Option<i32>, // Example of handling missing numeric values
}

// Step 1: Read CSV file
fn read_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;

    println!("Reading CSV file:");
    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }

    Ok(())
}

// Step 2: Clean CSV file
fn clean_csv(file_path: &str) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut reader = ReaderBuilder::new().from_path(file_path)?;
    let mut cleaned_data = Vec::new();

    let re = Regex::new(r"[^\w\s]").unwrap(); // Regex to remove non-alphanumeric characters

    for result in reader.records() {
        let record = result?;
        let cleaned_record: Vec<String> = record
            .iter()
            .map(|field| {
                let sanitized = re.replace_all(field, "").to_string();
                if sanitized.is_empty() { "N/A".to_string() } else { sanitized }
            })
            .collect();
        cleaned_data.push(cleaned_record);
    }

    Ok(cleaned_data)
}

// Step 3: Analyze CSV file
fn analyze_csv(file_path: &str) -> Result<(), Box<dyn Error>> {
    println!("Analyzing CSV file:");

    // Using Polars for advanced analysis
    let df = CsvReader::from_path(file_path)?
        .infer_schema(None)
        .has_header(true)
        .finish()?;

    println!("DataFrame Summary:");
    use polars::prelude::*;

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
        let mut reader = ReaderBuilder::new().from_path(file_path)?;
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

// Step 4: Write cleaned data to a new CSV file
fn write_csv(file_path: &str, records: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let mut writer = WriterBuilder::new().from_path(file_path)?;

    for record in records {
        writer.write_record(&record)?;
    }

    writer.flush()?;
    println!("Cleaned data written to: {}", file_path);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "influencers_september.csv";
    let cleaned_file_path = "cleaned_output.csv";

    // Step 1: Read and print CSV data
    read_csv(file_path)?;

    // Step 2: Clean data
    let cleaned_data = clean_csv(file_path)?;

    // Step 3: Analyze data
    analyze_csv(file_path)?;

    // Step 4: Write cleaned data to a new CSV file
    write_csv(cleaned_file_path, cleaned_data)?;

    Ok(())
}