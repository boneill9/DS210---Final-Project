use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

// Helper to calculate Euclidean distance
pub fn euclidean_distance(p1: &Point, p2: &Point) -> f64 {
    ((p1.x - p2.x).powi(2) + (p1.y - p2.y).powi(2)).sqrt()
}

// K-means function
pub fn k_means(points: Vec<Point>, k: usize, max_iterations: usize) -> Vec<Vec<Point>> {
    let mut rng = thread_rng();

    // Randomly initialize centroids
    let mut centroids: Vec<Point> = points.choose_multiple(&mut rng, k).cloned().collect();
    let mut clusters: Vec<Vec<Point>> = vec![Vec::new(); k];

    for _ in 0..max_iterations {
        // Clear previous clusters
        for cluster in &mut clusters {
            cluster.clear();
        }

        // Assign points to nearest centroid
        for point in &points {
            let mut min_distance = f64::MAX;
            let mut nearest_centroid_idx = 0;

            for (i, centroid) in centroids.iter().enumerate() {
                let distance = euclidean_distance(point, centroid);
                if distance < min_distance {
                    min_distance = distance;
                    nearest_centroid_idx = i;
                }
            }

            clusters[nearest_centroid_idx].push(point.clone());
        }

        // Update centroids
        for (i, cluster) in clusters.iter().enumerate() {
            if !cluster.is_empty() {
                let sum_x: f64 = cluster.iter().map(|p| p.x).sum();
                let sum_y: f64 = cluster.iter().map(|p| p.y).sum();
                let len = cluster.len() as f64;
                centroids[i] = Point {
                    x: sum_x / len,
                    y: sum_y / len,
                };
            }
        }
    }

    clusters
}