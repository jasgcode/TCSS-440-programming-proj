use std::time::{Instant, Duration};
use std::error::Error;
use plotters::prelude::*;

use crate::DFA;

pub fn run_performance_test(dfa: &DFA, test_cases: &[&str], iterations: usize) -> Result<Vec<(String, Duration)>, Box<dyn Error>> {
    let mut results = Vec::new();

    for &test_case in test_cases {
        let start = Instant::now();
        for _ in 0..iterations {
            dfa.process(test_case, false)?;
        }
        let elapsed = start.elapsed();
        let avg_time = elapsed / iterations as u32;
        
        println!("Test case '{}': Avg time: {:?}", test_case, avg_time);
        results.push((test_case.to_string(), avg_time));
    }

    Ok(results)
}

pub fn generate_performance_graph(results: &[(String, Duration)]) -> Result<(), Box<dyn Error>> {
    let root = BitMapBackend::new("dfa_performance.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_time = results.iter().map(|(_, time)| time.as_nanos()).max().unwrap() as f64;

    let mut chart = ChartBuilder::on(&root)
        .caption("DFA Performance", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(35)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..results.len(),
            0f64..max_time * 1.1
        )?;

    chart
        .configure_mesh()
        .x_labels(results.len())
        .x_label_formatter(&|x| {
            if let Some((input, _)) = results.get(*x) {
                input.clone()
            } else {
                String::new()
            }
        })
        .draw()?;

    chart
        .draw_series(
            results.iter().enumerate().map(|(i, (_, time))| {
                let time_ns = time.as_nanos() as f64;
                Rectangle::new([(i, 0.0), (i + 1, time_ns)], BLUE.filled())
            }),
        )?
        .label("Processing Time (ns)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}