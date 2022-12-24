use core::f32;
use std::{fs, vec};
use plotly::common::Mode;
use plotly::{Layout, Plot, Scatter, }; 

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

fn draw_plot(x: &Vec<f32>, y: &Vec<f32>, line: (Vec<f32>, Vec<f32>)) {
  let points = Scatter::new(x.to_vec(), y.to_vec())
    .name("trace1")
    .mode(Mode::Markers);
  
  let line = Scatter::new(line.0, line.1)
    .name("trace2")
    .mode(Mode::Lines);

  let mut plot = Plot::new();
  plot.add_trace(points);
  plot.add_trace(line);
  
  let layout = Layout::new().title("<b>Rusty Linear Regressions</b>".into());
  plot.set_layout(layout);

  plot.show();
}

fn loss(x: &Vec<f32>, y: &Vec<f32>, weight: &f32, bias: &f32) -> f32 {
  let mut loss: f32 = 0.0;
  let n = x.len();

  for i in 0..n {
    loss += (y[i] - (weight*x[i] + bias)) * (y[i] - (weight*x[i] + bias));
  };

  loss/n as f32
}

fn learn(x: &Vec<f32>, y: &Vec<f32>,mut weight: f32, mut bias: f32, learning_rate: f32, iterations: usize) -> (f32, f32) {


  let mut bias_gradient = 0.0;
  let mut weight_gradient = 0.0;

  for _ in 0..iterations {

    let len = x.len();
    let n =  x.len() as f32;

    for j in 0..len {
      let current_x = x[j];
      let current_y = y[j];

      bias_gradient += -(2.0/n) * (current_y - (weight * current_x + bias));
      weight_gradient += -(2.0/n) * current_x * (current_y - weight * current_x + bias);
    }

    (bias, weight) = (bias - learning_rate * bias_gradient,
       weight - learning_rate * weight_gradient); 
  }

  return (weight, bias);

}

fn main() {
  
  
  let (x, y) = load_data();
  let (mut line_x,mut line_y): (Vec<f32>, Vec<f32>) = (vec![], vec![]);

  let  ( start_weight, start_bias) = (0.0, 0.0);

  let learning_rate = 0.00001;
  let iterations = 1000;

  let (weight, bias) = learn(&x, &y, start_weight, start_bias, learning_rate, iterations);

  for x in &x {
    line_x.push(*x);
    line_y.push(x*weight + bias);
  } 

  let total_loss = loss(&x, &y, &weight, &bias);
  println!("loss after: {}", total_loss);

  draw_plot(&x, &y, (line_x, line_y));
  
}

