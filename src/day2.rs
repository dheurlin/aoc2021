pub enum Instruction {
  Forward(u8),
  Up(u8),
  Down(u8),
}

use Instruction::*;

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Instruction> {
  input
    .lines()
    .map(|l| {
      let mut parts = l.split(' ');
      let instr_str = parts.next().unwrap();
      let num = parts.next().and_then(|s| s.parse().ok()).unwrap();
      match instr_str {
        "forward" => Forward(num),
        "up"      => Up(num),
        "down"    => Down(num),
        _         => unreachable!("Instruction not implemented!")
      }
    })
    .collect()
}


#[aoc(day2, part1)]
pub fn part1(input: &Vec<Instruction>) -> u32 {

  struct SubState {
    depth: u32,
    horiz: u32,
  }

  let final_state = input
    .iter()
    .fold(SubState { depth: 0, horiz: 0 }, |state, instr| {
      match *instr {
        Forward(n) => SubState { horiz: state.horiz + (n as u32), ..state },
        Up(n)      => SubState { depth: state.depth - (n as u32), ..state },
        Down(n)    => SubState { depth: state.depth + (n as u32), ..state },
      }
    });

  final_state.depth * final_state.horiz

}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Instruction>) -> u32 {

  struct SubState {
    depth: u32,
    horiz: u32,
    aim  : u32,
  }

  let final_state = input
    .iter()
    .fold(SubState { depth: 0, horiz: 0, aim: 0 }, |state, instr| {
      match *instr {
        Forward(n) => SubState {
          depth: state.depth + state.aim * (n as u32),
          horiz: state.horiz + (n as u32),
          ..state
        },
        Up(n)   => SubState { aim: state.aim - (n as u32), ..state },
        Down(n) => SubState { aim: state.aim + (n as u32), ..state },
      }
    });

  final_state.depth * final_state.horiz

}
