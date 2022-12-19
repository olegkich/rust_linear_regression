use std::{fs, vec};

fn main() {
  let (x, y) = load_data();
  
}

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
