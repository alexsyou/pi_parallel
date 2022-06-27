use plotters::prelude::*;

use crate::PRECISION;

fn plot() {
    let root_area = BitMapBackend::new(format!("images/pi_comparison_{}.png", PRECISION), (600, 400))
          .into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let mut ctx = ChartBuilder::on(&root_area)
          .set_label_area_size(LabelAreaPosition::Left, 40)
          .set_label_area_size(LabelAreaPosition::Bottom, 40)
          .caption("Line Plot Demo", ("sans-serif", 40))
          .build_cartesian_2d(1..64, 0..100)
          .unwrap();

    ctx.configure_mesh().draw().unwrap();

    ctx.draw_series(
          LineSeries::new((-10..=10).map(|x| (x, x* x)), &GREEN)
    ).unwrap();
}
