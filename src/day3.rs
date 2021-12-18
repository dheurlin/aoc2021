extern crate itertools;
extern crate transpose;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> usize {

  let (γ, ε) = lines_to_cols(input)
    .iter()
    .map(|line| {
      let ones = line.iter().filter(|&chr| *chr == '1').count();
      let zeros = line.iter().count() - ones;
      (zeros, ones)
    })
    .fold((0, 0),
      |(curr_γ, curr_ε), (zeros, ones)| {
        let el = if zeros > ones { 0 } else { 1 };
        (curr_γ * 2 + el, curr_ε * 2 + (1 - el))
      }
    );
  ε * γ
}

fn lines_to_cols(s: &str) -> Vec<Vec<char>> {
  let mut lines = s.lines();
  let num_cols = lines.by_ref().peekable().peek().unwrap().chars().count();
  let mut res = vec![vec!['0'; s.lines().count()];num_cols];
    s.lines().enumerate().for_each(|(row, line)| {
      line.chars().enumerate().for_each(|(col, chr)| {
        res[col][row] = chr;
      })
    });

  res
}

fn filter_input<F>(mut remaining: Vec<Vec<char>>, keep_fun: F) -> Vec<char>
where
  F: Fn(usize, usize) -> bool,
{
  let num_cols = remaining.iter().next().unwrap().iter().count();
  let mut range = 0..=num_cols;
  let res = loop {
    match range.next() {
      None => panic!("This shouldn't happen mate..."),
      Some(col) => {
        if remaining.iter().count() == 1 {
          break &remaining[0];
        }

        let (ones, zeroes) = remaining
          .iter()
          .fold((0, 0), |(curr_ones, curr_zeroes), line|{
            if line[col] == '1' { (curr_ones + 1, curr_zeroes) } else { (curr_ones, curr_zeroes + 1) }
          });

        if keep_fun(ones, zeroes) {
          remaining = remaining.into_iter().filter(|line| line[col] == '1').collect::<Vec<Vec<_>>>();
        } else {
          remaining = remaining.into_iter().filter(|line| line[col] == '0').collect::<Vec<Vec<_>>>();
        }
      },
    }
  };

  res.to_vec()
}

fn bin_digits_to_number(digits: Vec<char>) -> usize {
  digits
    .into_iter()
    .fold(0, |curr, digit| {
      curr * 2 + if digit == '1' { 1 } else { 0 }
    })
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> usize {

  let remaining_oxy = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();
  let remaining_co2 = input.lines().map(|line| line.chars().collect::<Vec<_>>()).collect::<Vec<Vec<_>>>();

  let oxy = bin_digits_to_number(filter_input(remaining_oxy, |ones, zeroes| ones >= zeroes));
  let co2 = bin_digits_to_number(filter_input(remaining_co2, |ones, zeroes| zeroes > ones));

  println!("oxy: {:?}, co2: {:?} ", oxy, co2);

  oxy * co2

}
