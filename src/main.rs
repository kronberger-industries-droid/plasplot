use std::path::{Path, PathBuf};

use charming::{
    component::Axis,
    element::{Label, MarkLineDataType, MarkPoint, MarkPointData},
    series::Line,
    Chart, ImageRenderer,
};
use csv::{Position, ReaderBuilder};
use polyfit_rs::polyfit_rs;

#[derive(Debug)]
struct PlasmaParams {
    electron_temp: f64,
    electron_density: f64,
    density: f64,
}

impl PlasmaParams {
    fn new(i_sp: f64, i_sn: f64, d_i: f64, p: f64) -> Self {
        let electron_temp = ((i_sp * i_sn) / (i_sp + i_sn)) / d_i;
        let electron_density = 3e21 * (i_sp + i_sn) / electron_temp.sqrt();
        let density = electron_density / (2.5e22 * p);
        Self {
            electron_temp,
            electron_density,
            density,
        }
    }
}

fn read_and_plot(filepath: PathBuf, renderer: &mut ImageRenderer) {
    let mut reader = ReaderBuilder::new()
        .delimiter(b' ')
        .from_path(filepath)
        .expect("Could not open file");

    let records = reader.records().skip(1);

    let mut data = Vec::new();

    for result in records {
        let record = result.expect("Could not parse record");
        // Parse the first two columns as f64 values
        let x: f64 = record.get(0).unwrap().parse().unwrap_or_default();
        let y: f64 = record.get(2).unwrap().parse().unwrap_or_default();
        data.push(vec![x, y]);
    }

    // Get a slice of the tail of the data
    let tail_slice = &data[500..];

    // Unzip the slice into x and y vectors
    let (x, y): (Vec<f64>, Vec<f64>) = tail_slice.iter().map(|v| (v[0], v[1])).unzip();

    // Perform the polyfit on the tail slice
    let fit_tail = polyfit_rs::polyfit(&x, &y, 1).expect("Could not fit the curve!");

    // Compute the fitted values for the tail slice
    let fit_data_tail: Vec<Vec<f64>> = data
        .iter()
        .map(|record| {
            let x_val = record[0];
            let y_val = x_val * fit_tail[1] + fit_tail[0];
            vec![x_val, y_val]
        })
        .collect();

    // Get a slice of the front of the data
    let front_slice = &data[..100];

    // Unzip the slice into x and y vectors
    let (x_front, y_front): (Vec<f64>, Vec<f64>) = front_slice.iter().map(|v| (v[0], v[1])).unzip();

    // Perform the polyfit on the front slice
    let fit_front =
        polyfit_rs::polyfit(&x_front, &y_front, 1).expect("Could not fit the front curve!");

    // Compute the fitted values for the front slice
    let fit_data_front: Vec<Vec<f64>> = data
        .iter()
        .map(|record| {
            let x_val = record[0];
            let y_val = x_val * fit_front[1] + fit_front[0];
            vec![x_val, y_val]
        })
        .collect();

    let d_i = match data.iter().find(|v| v[0] == 0.0) {
        Some(point) => point[1],
        None => {
            eprintln!("No point at 0.0 found! Returning 0.0");
            0.0
        }
    };

    let plasma = PlasmaParams::new(fit_front[0], fit_tail[0], d_i, 0.6);

    println!("{:?}", plasma);

    // Create a chart using the CSV data
    let chart = Chart::new()
        .x_axis(Axis::new().name("U in V"))
        .y_axis(Axis::new().name("Ig in muA"))
        .series(Line::new().show_symbol(false).data(data))
        .series(
            Line::new()
                .show_symbol(false)
                .data(fit_data_front)
                .mark_point(MarkPoint::new().data(vec![MarkPointData::new()
                        .name("Something")
                        .x_axis(0.0)
                        .y_axis(fit_front[0]).value((fit_front[0]*10.0).round()/10.0)])),
        )
        .series(
            Line::new()
                .show_symbol(false)
                .data(fit_data_tail)
                .mark_point(MarkPoint::new().data(vec![MarkPointData::new()
                        .name("Something")
                        .x_axis(0.0)
                        .y_axis(fit_tail[0]).value((fit_tail[0]*10.0).round()/10.0)])),
        );

    renderer
        .save(&chart, "src/plot.svg")
        .unwrap_or_else(|e| eprintln!("Could not render file: {}", e));
}

fn main() {
    let mut renderer = ImageRenderer::new(1000, 800);
    let path = Path::new("data/data_5-2-4.csv").to_path_buf();

    read_and_plot(path, &mut renderer);
}
