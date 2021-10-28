use naca::{NACAAirfoil, NACA4};
use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("NACA.png", (1920, 1080)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut id = String::new();
    std::io::stdin().read_line(&mut id)?;
    let id = id.trim_end();
    let title = format!("NACA {}", id);
    let naca4: NACA4 = id.parse()?;

    let yus: Vec<_> = (0..=10000)
        .map(|x| x as f32 / 10000.0)
        .map(|x| (naca4.xu(x), naca4.yu(x)))
        .collect();
    let yls = (0..=10000)
        .map(|x| x as f32 / 10000.0)
        .map(|x| (naca4.xl(x), naca4.yl(x)));
    let high = yus
        .iter()
        .max_by(|(_x1, y1), (_x2, y2)| y1.abs().partial_cmp(&y2.abs()).unwrap())
        .unwrap()
        .1
        + 0.1;

    let mut chart = ChartBuilder::on(&root)
        .caption(title, ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..1f32, -high..high)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(LineSeries::new(yus, &RED))?
        .label("Upper")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .draw_series(LineSeries::new(yls, &RED))?
        .label("Lower")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}
