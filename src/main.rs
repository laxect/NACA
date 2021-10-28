use gnuplot::{
    AutoOption, AxesCommon, Figure,
    PlotOption::{Caption, Color, LineWidth},
};
use naca::{NACAAirfoil, NACA4};
use std::env::args;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let id = args().nth(1).unwrap();
    let id = id.trim_end();
    let title = format!("NACA {}", id);
    let naca4: NACA4 = id.parse()?;

    let (xu, yu): (Vec<f32>, Vec<f32>) = (0..=10000)
        .map(|x| x as f32 / 10000.0)
        .map(|x| (naca4.xu(x), naca4.yu(x)))
        .unzip();
    let (xl, yl): (Vec<f32>, Vec<f32>) = (0..=10000)
        .map(|x| x as f32 / 10000.0)
        .map(|x| (naca4.xl(x), naca4.yl(x)))
        .unzip();

    let (xm, ym): (Vec<f32>, Vec<f32>) = (0..=10000)
        .map(|x| x as f32 / 10000.0)
        .map(|x| (x, naca4.yc(x)))
        .unzip();

    let mut figure = Figure::new();
    figure.set_title(&title);
    figure
        .axes2d()
        .lines(xu, yu, &[Color("red"), LineWidth(2.5)])
        .lines(xl, yl, &[Color("red"), LineWidth(2.5)])
        .lines(xm, ym, &[Caption("Mid"), Color("purple"), LineWidth(2.5)])
        .lines([0, 1], [0, 0], &[Color("black"), LineWidth(1.0)])
        .set_aspect_ratio(AutoOption::Fix(0.5))
        .set_y_range(AutoOption::Fix(-0.25), AutoOption::Fix(0.25))
        .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(1.0));
    figure.save_to_png("NACA.png", 1920, 1080)?;
    Ok(())
}
