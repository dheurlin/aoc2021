#[aoc(day1, part1)]
pub fn part1(input: &str) -> u32 {
  input
    .lines()
    .map(|l| l.parse().unwrap())
    .fold((0, u32::MAX), |(sum, prev), next| {
      (sum + if next > prev { 1 } else { 0 }, next)
    })
    .0
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
  let mut nums = input.lines().map(|l| l.parse().unwrap());

  struct WindowState {
    sum: u32,
    avg: f32,
    prev: u32,
    curr: u32,
  }

  let fst = nums.next().unwrap();
  let snd = nums.next().unwrap();

  nums
    .fold(
      WindowState {
        sum: 0,
        avg: f32::MAX,
        prev: fst,
        curr: snd
      },
      |state, next| {
        let new_avg = ((state.prev + state.curr + next) as f32) / 3.0;

        WindowState {
          sum: state.sum + if new_avg > state.avg { 1 } else { 0 },
          avg: new_avg,
          prev: state.curr,
          curr: next,
        }
    })
    .sum
}
