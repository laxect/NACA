use std::env::args;

use gnuplot::{
    AutoOption, AxesCommon, Figure,
    PlotOption::{Caption, Color},
};
use naca::{NACAAirfoil, NACA4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = args().nth(1).unwrap();
    let id = id.trim_end();
    let title = format!("NACA {}", id);
    let naca4: NACA4 = id.parse()?;

    let (xu, yu): (Vec<f32>, Vec<f32>) = (0..=1000)
        .map(|x| x as f32 / 1000.0)
        .map(|x| (naca4.xu(x), naca4.yu(x)))
        .unzip();
    let (xl, yl): (Vec<f32>, Vec<f32>) = (0..=100)
        .map(|x| x as f32 / 100.0)
        .map(|x| (naca4.xl(x), naca4.yl(x)))
        .unzip();

    let mut figure = Figure::new();
    figure.set_title(&title);
    figure
        .axes2d()
        .lines(xu, yu, &[Caption("Upper"), Color("red")])
        .lines(xl, yl, &[Caption("Lower"), Color("red")])
        .set_aspect_ratio(AutoOption::Fix(0.25))
        .set_y_range(AutoOption::Fix(-0.25), AutoOption::Fix(0.25))
        .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(1.0));
    figure.show()?;
    Ok(())
}
