use csv::{ReaderBuilder, WriterBuilder};
use regex::Regex;
use std::error::Error;
use std::fs::File;

pub struct CsvFileProcessor;

impl CsvFileProcessor {
    pub fn clean_csv(&self, input_path: &str, output_path: &str) -> Result<(), Box<dyn Error>> {
        // Create a regex to clean the numbers
        let re = Regex::new(r"(?i)(\d+(?:\.\d+)?)([mk])?").unwrap();
        
        // Open the input CSV file for reading
        let input_file = File::open(input_path)?;
        let mut rdr = ReaderBuilder::new().has_headers(true).from_reader(input_file);
        
        // Create the output CSV file for writing (we're overwriting here)
        let output_file = File::create(output_path)?;
        let mut wtr = WriterBuilder::new().has_headers(true).from_writer(output_file);
        
        // Get the headers from the input file
        let headers = rdr.headers()?.clone();
        wtr.write_record(&headers)?;  // Write headers to the output file

        // Iterate over each record (row) in the CSV file
        for result in rdr.records() {
            let record = result?;
            let cleaned_record: Vec<String> = record.iter()
                .map(|field| {
                    // Process each field to clean up the numbers
                    if let Some(caps) = re.captures(field) {
                        // Parse the numeric value as a float
                        let value = caps[1].parse::<f64>().unwrap_or(0.0);
                        
                        // Handle multipliers for M and K (millions and thousands)
                        let multiplier = match caps.get(2).map(|m| m.as_str()) {
                            Some("M") | Some("m") => 100_000.0,
                            Some("K") | Some("k") => 1_000.0,
                            _ => 1.0,
                        };
                        
                        // Apply multiplier and convert to string
                        (value * multiplier).to_string()
                    } else {
                        field.to_string() // Non-numeric fields remain unchanged
                    }
                })
                .collect();

            // Write the cleaned record to the output file
            wtr.write_record(cleaned_record)?;
        }

        Ok(())
    }
}
