use gnuplot::{
    AutoOption, AxesCommon, Figure,
    PlotOption::{Caption, Color, LineWidth},
};
use naca::{NACAAirfoil, NACA4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..10 {
        for j in 0..100 {
            let num = i * 100 + j;
            println!("-- {}", num);
            let m = i as f32 / 100.0;
            let p = j as f32 / 100.0;
            let naca4 = NACA4 { m, p, t: 0.12 };

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
            figure
                .axes2d()
                .lines(xu, yu, &[Color("red"), LineWidth(2.5)])
                .lines(xl, yl, &[Color("red"), LineWidth(2.5)])
                .lines(xm, ym, &[Caption("Mid"), Color("purple"), LineWidth(2.5)])
                .lines([0, 1], [0, 0], &[Color("black"), LineWidth(1.0)])
                .set_aspect_ratio(AutoOption::Fix(0.5))
                .set_y_range(AutoOption::Fix(-0.25), AutoOption::Fix(0.25))
                .set_x_range(AutoOption::Fix(0.0), AutoOption::Fix(1.0));
            figure.save_to_png(format!("NACA {:0>4}.png", num), 1920, 1080)?;
        }
    }

    Ok(())
}
