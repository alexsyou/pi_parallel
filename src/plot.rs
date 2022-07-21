use plotters::prelude::*;

use crate::PRECISION;

pub fn plot(
    si_time: f64,
    mpsc_vec: &Vec<f64>,
    rayon_vec: &Vec<f64>,
    crossbeam_vec: &Vec<f64>,
    flume_vec: &Vec<f64>,
) {
    let root_area = BitMapBackend::new("images/pi_comparison.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();

    let max_size = ((PRECISION >> 32) * 4) as f64;

    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .caption("Pi Comparison", ("sans-serif", 40))
        .build_cartesian_2d(0u32..72u32, 0f64..max_size)
        .unwrap();

    ctx.configure_mesh().draw().unwrap();

    let mpsc_vec: Vec<(u32, f64)> = mpsc_vec
        .into_iter()
        .enumerate()
        .map(|(idx, time)| (u32::pow(2, (idx + 1) as u32), f64::from(*time)))
        .collect();

    let rayon_vec: Vec<(u32, f64)> = rayon_vec
        .into_iter()
        .enumerate()
        .map(|(idx, time)| (u32::pow(2, (idx + 1) as u32), f64::from(*time)))
        .collect();

    let crossbeam_vec: Vec<(u32, f64)> = crossbeam_vec
        .into_iter()
        .enumerate()
        .map(|(idx, time)| (u32::pow(2, (idx + 1) as u32), f64::from(*time)))
        .collect();

    let flume_vec: Vec<(u32, f64)> = flume_vec
        .into_iter()
        .enumerate()
        .map(|(idx, time)| (u32::pow(2, (idx + 1) as u32), f64::from(*time)))
        .collect();

    ctx.draw_series(PointSeries::of_element(
        vec![(1, si_time)],
        1,
        &BLUE,
        &|c, _s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), 2, st.filled());
        },
    ))
    .unwrap()
    .label("sequential")
    .legend(|(x, y)| Circle::new((x - 10, y), 5, BLUE.filled()));

    ctx.draw_series(PointSeries::of_element(
        mpsc_vec,
        6,
        &YELLOW,
        &|c, _s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), 2, st.filled());
        },
    ))
    .unwrap()
    .label("mpsc")
    .legend(|(x, y)| Circle::new((x - 10, y), 5, YELLOW.filled()));

    ctx.draw_series(PointSeries::of_element(
        rayon_vec,
        6,
        &BLACK,
        &|c, _s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), 2, st.filled());
        },
    ))
    .unwrap()
    .label("rayon")
    .legend(|(x, y)| Circle::new((x - 10, y), 5, BLACK.filled()));

    ctx.draw_series(PointSeries::of_element(
        crossbeam_vec,
        6,
        &GREEN,
        &|c, _s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), 2, st.filled());
        },
    ))
    .unwrap()
    .label("crossbeam")
    .legend(|(x, y)| Circle::new((x - 10, y), 5, GREEN.filled()));

    ctx.draw_series(PointSeries::of_element(
        flume_vec,
        6,
        &MAGENTA,
        &|c, _s, st| {
            return EmptyElement::at(c) + Circle::new((0, 0), 2, st.filled());
        },
    ))
    .unwrap()
    .label("flume")
    .legend(|(x, y)| Circle::new((x - 10, y), 5, MAGENTA.filled()));

    ctx.configure_series_labels()
        .position(SeriesLabelPosition::UpperRight)
        .margin(20)
        .legend_area_size(5)
        .border_style(CYAN)
        .background_style(CYAN.mix(0.1))
        .label_font(("sans-serif", 20))
        .draw()
        .unwrap();
}
