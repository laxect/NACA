use gnuplot::{
    AutoOption, AxesCommon, Figure,
    PlotOption::{Caption, Color, LineWidth},
};
use naca::{NACAAirfoil, NACA4};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ths = Vec::new();
    for i in 1..10 {
        let i = i;
        let thread = std::thread::spawn(move || {
            for j in 0..100 {
                let num = (i - 1) * 100 + j;
                println!("-- {}", num);
                let m = i as f32 / 100.0;
                let p = j as f32 / 100.0;
                let naca4 = NACA4 { m, p, t: 0.12 };
                let ((u, l), m): ((Vec<_>, Vec<_>), Vec<_>) = (0..=10000)
                    .map(|x| x as f32 / 10000.0)
                    .map(|x| naca4.all(x))
                    .unzip();

                let (xu, yu): (Vec<_>, Vec<_>) = u.into_iter().unzip();
                let (xl, yl): (Vec<_>, Vec<_>) = l.into_iter().unzip();
                let (xm, ym): (Vec<_>, Vec<_>) = m.into_iter().unzip();

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
                figure
                    .save_to_png(format!("NACA{:0>4}.png", num), 1920, 1080)
                    .ok();
            }
        });
        ths.push(thread);
    }
    for th in ths {
        th.join().unwrap();
    }
    Ok(())
}
