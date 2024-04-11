use super::*;
use plotters::prelude::*;

// const IMAGE_DIM : u32 = 800; // 1600 x 1600 dimensions
pub fn draw_history(function : &dyn Function, history : &History) {   
    // let function_space = function_space(function);
    // function_space.save("outputs/function.png").unwrap();
    
}

fn plot_function(function: &'static dyn Function, plotname: &str) {
    let file_name = format!("outputs/{}.png", plotname);
    let drawing_area = BitMapBackend::new(file_name.as_str(), (800, 800))
            .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();

    let [[x_min, x_max], [y_min, y_max]] = function.domain();
    let mut chart = ChartBuilder::on(&drawing_area)
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max).unwrap();
    chart.configure_mesh().draw().unwrap();

    let plotting_area = chart.plotting_area();
    let range = plotting_area.get_pixel_range();

    let max = function_set(function, range.0.end - range.0.start, range.1.end - range.1.start)
        .into_iter()
        .map(|(_,_,a)| a)
        .max_by(|a, b| a.total_cmp(&b)).unwrap();
    
    for (x,y,val) in function_set(function, range.0.end - range.0.start, range.1.end - range.1.start) {
        plotting_area.draw_pixel((x,y), &MandelbrotHSL::get_color(val / max)).unwrap()
    }

    drawing_area.present().unwrap();
}
fn function_set(
    function : &'static dyn Function,
    plotwidth: i32,
    plotheight: i32, 
) -> impl Iterator<Item = (f64, f64, f64)> {
    let [[x_min, x_max], [y_min, y_max]] = function.domain();
    let step = (
        (x_max - x_min) / plotwidth as f64,
        (y_max - y_min) / plotheight as f64,
    );
    (0..(plotwidth * plotheight)).map(move |k| {
        let c = (
            x_min + step.0 * (k % plotwidth) as f64,
            y_min + step.1 * (k / plotwidth) as f64,
        );
        let val = function.eval(c).unwrap();
        (c.0, c.1, val)
    })
}
mod test {
    
    use std::ops::Range;
    use crate::functions::parabolla::Parabolla;
    use self::{rastrigin::Rastrigin, rosenbrock::Rosenbrock};

    use super::{function_set, functions::*, plot_function};
    #[test]
    fn parabolla() {
        plot_function(&Parabolla, "parabolla")
    }   

    #[test]
    fn rastrigin(){
        plot_function(&Rastrigin, "rastrigin")
    }

    #[test]
fn rosenbrock(){
        plot_function(&Rosenbrock, "rosenbrock")
    }
}