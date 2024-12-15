extern crate rand;
extern crate plotters;

use plotters::prelude::*;
use rand::Rng;

pub trait KMeans {
    fn kmeans(&self, k: usize, max_iters: usize) -> Vec<usize>
    where
        Self: Sized;
    fn initialize_centroids(&self, k: usize) -> Vec<Vec<f64>>;
    fn assign_labels(&self, centroids: &[Vec<f64>]) -> Vec<usize>;
    fn update_centroids(&self, labels: &[usize], k: usize) -> Vec<Vec<f64>>;
    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64;
}

// Implement the KMeans trait for Vec<Vec<f64>>
impl KMeans for Vec<Vec<f64>> {
    fn kmeans(&self, k: usize, max_iters: usize) -> Vec<usize>
    where
        Self: Sized
    {
        let mut centroids = self.initialize_centroids(k);
        let mut labels = vec![0; self.len()]; // Number of points in `self`

        for _ in 0..max_iters {
            let new_labels = self.assign_labels(&centroids);
            if new_labels == labels {
                break; // Algorithm has converged if labels don't change
            }
            labels = new_labels;
            centroids = self.update_centroids(&labels, k);
        }

        labels
    }

    fn initialize_centroids(&self, k: usize) -> Vec<Vec<f64>> {
        let mut rng = rand::thread_rng();
        let mut centroids = Vec::new();

        for _ in 0..k {
            let idx = rng.gen_range(0..self.len());
            centroids.push(self[idx].clone()); // Clone the randomly selected row
        }

        centroids
    }

    fn assign_labels(&self, centroids: &[Vec<f64>]) -> Vec<usize> {
        self.iter()
            .map(|point| {
                centroids
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        let dist_a = Self::euclidean_distance(&point.to_vec(), a);
                        let dist_b = Self::euclidean_distance(&point.to_vec(), b);
                        dist_a.partial_cmp(&dist_b).unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|(idx, _)| idx)
                    .unwrap_or_else(|| 0) // Fallback to some default index
            })
            .collect()
    }    

    fn update_centroids(&self, labels: &[usize], k: usize) -> Vec<Vec<f64>> {
        let mut new_centroids = vec![vec![0.0; self[0].len()]; k]; // Initialize centroids
        let mut counts = vec![0; k]; // Track the count of points in each cluster

        for (i, label) in labels.iter().enumerate() {
            for (j, &value) in self[i].iter().enumerate() {
                new_centroids[*label][j] += value; // Sum up points
            }
            counts[*label] += 1; // Increment count for the cluster
        }

        // Calculate the mean for each centroid
        for (centroid, count) in new_centroids.iter_mut().zip(counts.iter()) {
            for val in centroid.iter_mut() {
                *val /= *count as f64; // Average the values
            }
        }

        new_centroids
    }

    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64 {
        a.iter()
            .zip(b.iter())
            .map(|(x, y)| (x - y).powi(2))
            .sum::<f64>()
            .sqrt()
    }
}

// Plot function remains the same
pub fn plot_data(data: &Vec<Vec<f64>>, labels: &[usize], k: usize) -> Result<(), Box<dyn std::error::Error>> {
    // Calculate the min and max values for x and y axes
    let (min_x, max_x) = data.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), point| {
        (min.min(point[0]), max.max(point[0]))
    });

    let (min_y, max_y) = data.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(min, max), point| {
        (min.min(point[1]), max.max(point[1]))
    });

    // Make a little extra space around the data points
    let padding = 0.1;
    let x_range = (min_x - padding * (max_x - min_x))..(max_x + padding * (max_x - min_x));
    let y_range = (min_y - padding * (max_y - min_y))..(max_y + padding * (max_y - min_y));

    // Create the drawing area
    let root = BitMapBackend::new("kmeans_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Create a chart builder with the dynamic ranges
    let mut chart = ChartBuilder::on(&root)
        .caption("K-means Clustering", ("sans-serif", 40))
        .build_cartesian_2d(x_range, y_range)?;

    // Configure the mesh (axes)
    chart.configure_mesh().draw()?;

    // Plot the data points
    for (i, point) in data.iter().enumerate() {
        let x = point[0];
        let y = point[1];
        let label = labels[i];

        // Choose color based on label
        let color = match label {
            0 => RED.to_rgba(),
            1 => BLUE.to_rgba(),
            _ => GREEN.to_rgba(),
        };

        // Draw the point on the chart
        chart.draw_series(std::iter::once(Circle::new((x, y), 5, color)))?;
    }

    // Present the plot and save to file
    root.present()?;

    println!("Plot saved to 'kmeans_plot.png'.");
    Ok(())
}
