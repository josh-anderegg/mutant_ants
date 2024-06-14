use super::*;
use plotters::prelude::*;
use std::{fs, process::Command};

// The colors of each individual colony
const COLONY_COLOR_MAP : [RGBColor; 16] = [
    RGBColor{0: 255, 1: 255, 2: 255},
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

const WIDTH : u32 = 800;
const HEIGHT : u32 = 800;
const MARGIN : i32 = 20;
const LABEL_SIZE : i32 = 30;
pub enum Theme {
    Neon,
    Bright,
    Nord
}

pub fn draw_history(function : &'static dyn Function, history : &History, iteration_count: usize, theme : Theme, time: &String) {
    let colors = match theme {
        Theme::Neon => colormap_neon(),
        Theme::Bright => colormap_bright(),
        Theme::Nord => colormap_nord(),
    };
    fs::create_dir(format!("outputs/{}_{time}", function.name())).unwrap();

    let [[x_min, x_max], [y_min, y_max]] = function.domain();
    let range = (MARGIN+LABEL_SIZE..(WIDTH as i32 - MARGIN), MARGIN..(HEIGHT as i32 - MARGIN - LABEL_SIZE));

    let (plotwidth, plotheight) = (range.0.end - range.0.start, range.1.end - range.1.start);
    let set = function_set(function, plotwidth, plotheight); 
    let max = set
            .iter()
            .map(|((_,_),a)| *a)
            .max_by(|a, b| a.total_cmp(&b)).unwrap();
    
    
        'outer: for ite in 0..iteration_count{
        let file_name = format!("outputs/{}_{time}/{ite}.png", function.name());
        let drawing_area = BitMapBackend::new(file_name.as_str(), (WIDTH, HEIGHT))
            .into_drawing_area();
        drawing_area.fill(&WHITE).unwrap();
        
        let mut chart = ChartBuilder::on(&drawing_area)
            .margin(MARGIN)
            .x_label_area_size(LABEL_SIZE)
            .y_label_area_size(LABEL_SIZE)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .unwrap();

        chart.configure_mesh().draw().unwrap();
        let plotting_area = chart.plotting_area();
        for ((x,y),val) in set.iter(){
            plotting_area.draw_pixel((*x,*y), &colors.get_color(val / max)).unwrap()
        }
        
        for (id, colony) in history.iter().enumerate(){
            let workers = match colony.get(ite){
                Some(workers) => workers,
                None => break 'outer,
            }.iter();
            let births = workers.clone().filter_map(|(_, action)|{
                match action {
                    Action::Born(position) => Some(Circle::new(*position, 3, COLONY_COLOR_MAP[id].filled())),
                    _ => None,
                }
            });
            
            let stalls = workers.clone().filter_map(|(_, action)|{
                match action {
                    Action::Stall(position) => Some(Cross::new(*position, 3, COLONY_COLOR_MAP[id].filled())),
                    _ => None,
                }
            });
            
            let moves = workers.filter_map(|(_, action)| {
                match action {
                    Action::Move(from, to) => Some((*from , *to)),
                    _ => None
                }
            });
            chart.draw_series(births).unwrap();
            chart.draw_series(stalls).unwrap();
            for (from, to) in moves {
                chart.draw_series(LineSeries::new([from, to], COLONY_COLOR_MAP[id].filled()).point_size(2)).unwrap();
            }
        }
        drawing_area.present().unwrap();
    }
    combine_images(function.name(), time);
}

fn combine_images(name: &str, time: &String){
    // ffmpeg -framerate 30 -i outputs/temp/%d.png -c:v libx264 -r 30 -pix_fmt yuv420p output.mp4
    // rm outputs/temp/*.png 
    let ffmpeg_cmd = format!("ffmpeg -framerate 5 -i outputs/{name}_{time}/%d.png -r 5 -s 800x800 -pix_fmt  yuv420p outputs/{name}_{time}.gif\n
                                        rm -r outputs/{name}_{time}");
    // Run the command
    Command::new("sh")
       .arg("-c")
       .arg(&ffmpeg_cmd)
       .stdout(std::process::Stdio::null())
       .stderr(std::process::Stdio::null())
       .output()
       .expect("FFMPEG failed");
    
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
fn function_set(function : &'static dyn Function, plotwidth: i32, plotheight: i32, ) -> Vec<(Point, f64)> {
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
        ((c.0, c.1), val)
    }).collect()
}
