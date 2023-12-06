use itertools::Itertools;

// d = (t - w) * w
// => w = (t +- sqrt(t^2 - 4 d)) / 2
fn calc(times: &[usize], dists: &[usize]) -> usize {
  times.iter().zip(dists).map(|(&t, &d)| {
    let diff = ((t*t - 4*d) as f64).sqrt() as usize;
    let w = (t-diff) / 2;
    diff + ((t - w) * w > d) as usize
  }).product()
}

#[aoc::main(06)]
fn main(input: &str) -> (usize, usize) {
  let (times, dists) = input.lines().map(|l|
    l.split_whitespace()
      .skip(1)
      .map(|w| w.parse::<usize>().unwrap())
      .collect::<Vec<_>>()
  ).collect_tuple().unwrap();
  let p2time = times.iter().map(|w| w.to_string()).collect::<String>().parse().unwrap();
  let p2dist = dists.iter().map(|w| w.to_string()).collect::<String>().parse().unwrap();
  (calc(&times, &dists), calc(&[p2time], &[p2dist]))
}
