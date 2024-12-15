extern crate ndarray;
extern crate rand;
extern crate plotters;
extern crate polars;

use plotters::prelude::*;
use crate::k_means::rand::Rng;
use ndarray::Array2;


pub trait KMeans {
    fn kmeans(&self, k: usize, max_iters: usize) -> Vec<usize>
    where
        Self: Sized;
    fn initialize_centroids(&self, k: usize) -> Vec<Vec<f64>>;
    fn assign_labels(&self, centroids: &[Vec<f64>]) -> Vec<usize>;
    fn update_centroids(&self, labels: &[usize], k: usize) -> Vec<Vec<f64>>;
    fn euclidean_distance(a: &[f64], b: &[f64]) -> f64;
    

}

// Implement the KMeans trait for Array2<f64>
impl KMeans for Array2<f64> {
    fn kmeans(&self, k: usize, max_iters: usize) -> Vec<usize>
    where
        Self: Sized // Ensure Self is Sized
    {
        let mut centroids = self.initialize_centroids(k);
        let mut labels = vec![0; self.nrows()];  // `nrows` works now because self is an Array2<f64>

        for _ in 0..max_iters {
            let new_labels = self.assign_labels(&centroids);
            if new_labels == labels {
                break; // If labels don't change, the algorithm has converged
            }
            labels = new_labels;
            centroids = self.update_centroids(&labels, k);
        }

        labels
    }

    fn initialize_centroids(&self, k: usize) -> Vec<Vec<f64>> {
        let mut rng = rand::thread_rng();
        let mut centroids = Vec::new();
        let rows = self.nrows();

        for _ in 0..k {
            let idx = rng.gen_range(0..rows);
            centroids.push(self.row(idx).to_vec());
        }

        centroids
    }

    fn assign_labels(&self, centroids: &[Vec<f64>]) -> Vec<usize> {
        self.outer_iter()
            .map(|point| {
                centroids
                    .iter()
                    .enumerate()
                    .min_by(|(_, a), (_, b)| {
                        let dist_a = Self::euclidean_distance(&point.to_vec(), a);
                        let dist_b = Self::euclidean_distance(&point.to_vec(), b);
                        dist_a.partial_cmp(&dist_b).unwrap()
                    })
                    .map(|(idx, _)| idx)
                    .unwrap()
            })
            .collect()
    }

    fn update_centroids(&self, labels: &[usize], k: usize) -> Vec<Vec<f64>> {
        let mut new_centroids = vec![vec![0.0; self.ncols()]; k];
        let mut counts = vec![0; k];

        for (i, label) in labels.iter().enumerate() {
            new_centroids[*label].iter_mut().zip(self.row(i).iter()).for_each(|(c, &x)| {
                *c += x;
            });
            counts[*label] += 1;
        }

        for (centroid, count) in new_centroids.iter_mut().zip(counts.iter()) {
            for val in centroid.iter_mut() {
                *val /= *count as f64;
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


// Plot function remains the same as before
pub fn plot_data(data: &Array2<f64>, labels: &[usize], k: usize) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("kmeans_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("K-means Clustering", ("sans-serif", 40))
        .build_cartesian_2d(0.0..10.0, 0.0..10.0)?;

    chart.configure_mesh().draw()?;

    for i in 0..data.nrows() {
        let x = data[(i, 0)];
        let y = data[(i, 1)];
        let label = labels[i];

        let color = match label {
            0 => RED.to_rgba(),
            1 => BLUE.to_rgba(),
            _ => GREEN.to_rgba(),
        };

        chart.draw_series(std::iter::once(Circle::new((x, y), 5, color)))?;
    }

    root.present()?;
    Ok(())
}
