use super::*;
use plotters::prelude::*;
const COLONY_COLOR_MAP : [RGBColor; 16] = 
[
    RGBColor{0: 0, 1: 0, 2: 0},
    RGBColor{0: 0, 1: 255, 2: 0},
    RGBColor{0: 0, 1: 0, 2: 255},
    RGBColor{0: 0, 1: 255, 2: 255},
    RGBColor{0: 255, 1: 0, 2: 255},
    RGBColor{0: 255, 1: 255, 2: 0},
    RGBColor{0: 255, 1: 165, 2: 0},
    RGBColor{0: 128, 1: 0, 2: 128},
    RGBColor{0: 0, 1: 128, 2: 128},
    RGBColor{0: 0, 1: 0, 2: 128},
    RGBColor{0: 128, 1: 128, 2: 0},
    RGBColor{0: 128, 1: 0, 2: 0},
    RGBColor{0: 0, 1: 128, 2: 0},
    RGBColor{0: 255, 1: 0, 2: 128},
    RGBColor{0: 0, 1: 255, 2: 128},
    RGBColor{0: 135, 1: 206, 2: 235}
  ]; 

pub fn draw_history(function : &'static dyn Function, history : &History, iteration_count: usize) {   
    let colors = colormap_neon();
    let file_name = format!("outputs/example.gif");
    let drawing_area = BitMapBackend::gif(file_name.as_str(), (800, 800), 100)
        .unwrap()
        .into_drawing_area();

    let [[x_min, x_max], [y_min, y_max]] = function.domain();
    let dummy = ChartBuilder::on(&drawing_area)
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .unwrap();
    let range = dummy.plotting_area().get_pixel_range();
    let (plotwidth, plotheight) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let set = function_set(function, plotwidth, plotheight); 
    let max = set
            .iter()
            .map(|(_,_,a)| *a)
            .max_by(|a, b| a.total_cmp(&b)).unwrap();
    
    for ite in 0..iteration_count{
        drawing_area.fill(&WHITE).unwrap();
        let caption = format!("iteration: {}", ite);
        let mut chart = ChartBuilder::on(&drawing_area)
            .caption(caption, ("sans-serif", 20))
            .margin(20)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();
        chart.configure_mesh().draw().unwrap();

        let plotting_area = chart.plotting_area();
        
        for (x,y,val) in set.iter(){
            plotting_area.draw_pixel((*x,*y), &colors.get_color(val / max)).unwrap()
        }

        for (id, colony) in history.data.iter().enumerate(){
            let workers: Vec<(f64,f64)> = colony.get(ite).unwrap().iter()
                .map(|(_, pos)| *pos).collect();
            chart.draw_series(workers.iter()
                    .map(|(x,y)| Circle::new((*x,*y), 2, COLONY_COLOR_MAP[id].filled())))
                .unwrap();
        }
        drawing_area.present().unwrap();
    }
    
}

fn plot_function(function: &'static dyn Function, plotname: &str) {
    let file_name = format!("outputs/{}.png", plotname);
    let drawing_area = BitMapBackend::new(file_name.as_str(), (800, 800))
            .into_drawing_area();
    drawing_area.fill(&WHITE).unwrap();
    let colors = colormap_neon();
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
        plotting_area.draw_pixel((x,y), &colors.get_color(val / max)).unwrap();
    }

    drawing_area.present().unwrap();
}

fn colormap_nord() -> DerivedColorMap<RGBColor> {
    let blue = RGBColor{0: 37, 1: 51, 2: 67};
    let yellow = RGBColor{0: 235, 1: 203, 2: 139};

    DerivedColorMap::new(&[blue, yellow])
}
fn colormap_bright() -> DerivedColorMap<RGBColor> {
    let blue = RGBColor{0: 0, 1: 127, 2: 115};
    let green = RGBColor{0: 76, 1: 205, 2: 153};
    let orange = RGBColor{0: 255, 1: 199, 2: 0};
    let yellow = RGBColor{0: 255, 1: 244, 2: 85};

    DerivedColorMap::new(&[blue, green, orange, yellow])
}
fn colormap_neon() -> DerivedColorMap<RGBColor> {
    let blue = RGBColor{0: 29, 1: 43, 2: 83};
    let green = RGBColor{0: 126, 1: 37, 2: 83};
    let orange = RGBColor{0: 255, 1: 0, 2: 77};
    let yellow = RGBColor{0: 250, 1: 239, 2: 93};

    DerivedColorMap::new(&[blue, green, orange, yellow])
}
fn function_set(function : &'static dyn Function, plotwidth: i32, plotheight: i32, ) -> Vec<(f64, f64, f64)> {
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
    }).collect()
}

#[cfg(test)]
mod test {
    
    use crate::functions::rastrigin::Rastrigin;
    use crate::functions::parabolla::Parabolla;
    use crate::functions::rosenbrock::Rosenbrock;
    use crate::functions::ackley::Ackley;
    use super::*;
    
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

    #[test]
    fn ackley() {
        plot_function(&Ackley, "ackley")
    }   
}