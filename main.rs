use plotters::prelude::*;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::f64::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;

const THREADS: usize = 32; // Threads

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plot.png", (45000, 45000)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("Fractal", ("sans-serif", 100).into_font())
        .build_cartesian_2d(0f64..22000f64, 0f64..22000f64)?;

    chart.configure_mesh().draw()?;

    // The center and radius are adapted to the size of the canvas
    let center = (11000.0, 11000.0);
    let radius = 10000.0;

    // Generating Hexagon Points
    let points = (0..6)
        .map(|i| {
            (
                center.0 + radius * (PI / 3.0 * i as f64).cos(),
                center.1 + radius * (PI / 3.0 * i as f64).sin(),
            )
        })
        .collect::<Vec<_>>();

    let point_pairs = Arc::new(Mutex::new(Vec::new()));

    let handles: Vec<_> = (0..THREADS)
        .map(|_| {
            let points_clone = points.clone();
            let point_pairs_clone = Arc::clone(&point_pairs);

            thread::spawn(move || {
                let mut g = center;
                let mut prev_g = g;
                let mut rng = StdRng::from_entropy();


                for _ in 0..(750000000 / THREADS) {
                    prev_g = g;
                    let roll = rng.gen_range(0..6);
                    g.0 = (g.0 + points_clone[roll].0) / 2.0;
                    g.1 = (g.1 + points_clone[roll].1) / 2.0;

                    point_pairs_clone.lock().unwrap().push((prev_g, g));
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let point_pairs = Arc::try_unwrap(point_pairs).unwrap().into_inner().unwrap();

    // Drawing lines between points
    chart.draw_series(LineSeries::new(
        point_pairs.into_iter().map(|(start, end)| {
            (start, end)
        }),
        &BLACK,
    ))?;

    Ok(())
}
