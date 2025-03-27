// use std::any::Any;

// use charming::{
//     component::{Axis, Legend},
//     datatype::Dataset,
//     df,
//     element::{AxisType, LineStyle, SymbolSize},
//     series::{Line, Scatter},
//     Chart, HtmlRenderer,
// };
// use csv::{Reader, ReaderBuilder};

// fn main() {
//     let mut renderer = HtmlRenderer::new("test", 1000, 800);

//     // Create a CSV reader for a tab-separated file
//     let mut reader = ReaderBuilder::new()
//         .delimiter(b' ')
//         .from_path("src/test.csv")
//         .expect("Could not open file");

//     // Skip the first two lines (if needed)
//     let records = reader.records().skip(1);

//     // Collect CSV data into a vector of points (assuming two columns per row)
//     let mut data = Vec::new();
//     for result in records {
//         let record = result.expect("Could not parse record");
//         // Parse the first two columns as f64 values
//         let x: f64 = record.get(0).unwrap().parse().unwrap_or_default();
//         let y: f64 = record.get(4).unwrap().parse().unwrap_or_default();
//         data.push(vec![x, y]);
//     }
//     println!("{:?}", data);

//     // Create a chart using the CSV data
//     let chart = Chart::new()
//         .x_axis(Axis::new().align_ticks(true))
//         .y_axis(Axis::new().align_ticks(true))
//         .series(Scatter::new().symbol_size(20).data(df!(data)));

//     renderer
//         .save(&chart, "src/plot.html")
//         .unwrap_or_else(|e| eprintln!("Could not render file: {}", e));
// }
use charming::{component::Axis, series::Scatter, Chart, HtmlRenderer};

pub fn chart() -> Chart {
    Chart::new().x_axis(Axis::new()).y_axis(Axis::new()).series(
        Scatter::new().symbol_size(20).data(vec![
            vec![10.0, 8.04],
            vec![8.07, 6.95],
            vec![13.0, 7.58],
            vec![9.05, 8.81],
            vec![11.0, 8.33],
            vec![14.0, 7.66],
            vec![13.4, 6.81],
            vec![10.0, 6.33],
            vec![14.0, 8.96],
            vec![12.5, 6.82],
            vec![9.15, 7.2],
            vec![11.5, 7.2],
            vec![3.03, 4.23],
            vec![12.2, 7.83],
            vec![2.02, 4.47],
            vec![1.05, 3.33],
            vec![4.05, 4.96],
            vec![6.03, 7.24],
            vec![12.0, 6.26],
            vec![12.0, 8.84],
            vec![7.08, 5.82],
            vec![5.02, 5.68],
        ]),
    )
}

fn main() {
    let mut renderer = HtmlRenderer::new("test", 1000, 800);
    // Create a chart using the CSV data
    let chart = chart();
    renderer
        .save(&chart, "src/plot.html")
        .unwrap_or_else(|e| eprintln!("Could not render file: {}", e));
}
