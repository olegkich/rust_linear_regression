use std::{fs, vec};
use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter}; 

fn load_data() -> (Vec<f32>, Vec<f32>) {
  let s = fs::read_to_string("data.txt").expect("Read file");

  let mut splitted = s.split("],");

  let mut xs = format!("{}]", splitted.next().unwrap());
  xs = xs[2..xs.len() - 1].to_string();

  let mut ys = splitted.next().unwrap().to_string();
  ys = ys[2..ys.len() - 1].to_string();

  let x_iter = xs.split(", ");
  let y_iter = ys.split(", ");

  let mut x_data: Vec<f32> = vec![];
  let mut y_data: Vec<f32> = vec![];

  for x in x_iter {
    x_data.push(x.to_string().parse::<f32>().unwrap());
  }

  for y in y_iter {
    y_data.push(y.to_string().parse::<f32>().unwrap());
  }

  (x_data, y_data)
}

fn draw_plot(x: &Vec<f32>, y: &Vec<f32>) {
  let trace1 = Scatter::new(x.to_vec(), y.to_vec())
    .name("trace1")
    .mode(Mode::Markers);
  

  let mut plot = Plot::new();
  plot.add_trace(trace1);
  

  let layout = Layout::new().title("<b>Line and Scatter Plot</b>".into());
  plot.set_layout(layout);

  plot.show();
}

fn loss(x: &Vec<f32>, y: &Vec<f32>, w: f32, b: f32) -> f32 {
  let mut loss: f32 = 0.0;
  let n = x.len();

  for i in 0..n {
    loss += (y[i] - (w*x[i] + b)) * (y[i] - (w*x[i] + b));
  };

  loss/n as f32
}



fn main() {
  let (x, y) = load_data();
  draw_plot(&x, &y);
  let loss = loss(&x, &y, 1.2, -5.0);

  println!("loss: {}", loss);
}

