#[derive(Debug, Copy, Clone)]
struct Instruction {
    num_to_move: usize,
    start_stack: usize,
    end_stack: usize,
}

#[derive(Debug, Clone)]
struct CrateStacks {
    towers: Vec<Vec<char>>,
}

impl CrateStacks {
    fn exec_instruction_part_1(&mut self, instruction: Instruction) {
        // reverse start stack part
        let range_end = self.towers[instruction.start_stack].len();
        let range = range_end - instruction.num_to_move..range_end;
        self.towers[instruction.start_stack][range.clone()].reverse();
        // temporarily swap start stack out to avoid borrow issues
        let mut temp = vec![];
        std::mem::swap(&mut self.towers[instruction.start_stack], &mut temp);
        // copy to other stack
        self.towers[instruction.end_stack].extend_from_slice(&temp[range.clone()]);
        // truncate off of temp
        temp.truncate(range.start);
        // swap back
        std::mem::swap(&mut self.towers[instruction.start_stack], &mut temp);
    }

    fn exec_instruction_part_2(&mut self, instruction: Instruction) {
        let range_end = self.towers[instruction.start_stack].len();
        let range = range_end - instruction.num_to_move..range_end;
        // reverse start stack part
        // self.towers[instruction.start_stack][range.clone()].reverse();
        // temporarily swap start stack out to avoid borrow issues
        let mut temp = vec![];
        std::mem::swap(&mut self.towers[instruction.start_stack], &mut temp);
        // copy to other stack
        self.towers[instruction.end_stack].extend_from_slice(&temp[range.clone()]);
        // truncate off of temp
        temp.truncate(range.start);
        // swap back
        std::mem::swap(&mut self.towers[instruction.start_stack], &mut temp);
    }

    fn top_crate_word(&self) -> String {
        self.towers
            .iter()
            .map(|v| v.last().copied().unwrap_or(' '))
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let _input = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";
    let input = aoc_util::get_input_string(2022, 5)?;
    // println!("{}", input);

    // populate towers
    let mut towers = vec![vec![]; 9];
    let mut instruction_start = 0;
    for (num_lines, line) in input.lines().enumerate() {
        if line.starts_with("move") {
            instruction_start = num_lines;
            for tower in &mut towers {
                tower.reverse();
            }
            break;
        }
        for (col, c) in line
            .as_bytes()
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, c)| c.is_ascii_alphabetic())
        {
            let index = (col - 1) / 4;
            towers[index].push(c as char);
        }
    }
    let towers = CrateStacks { towers };
    // println!("{:#?}", towers);

    // parse instructions
    let mut instructions = vec![];
    for line in input.lines().skip(instruction_start) {
        let nums: Vec<_> = line
            .split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        instructions.push(Instruction {
            num_to_move: nums[0],
            start_stack: nums[1] - 1,
            end_stack: nums[2] - 1,
        });
    }

    // part 1
    let mut part_1_towers = towers.clone();
    for instruction in instructions.clone() {
        part_1_towers.exec_instruction_part_1(instruction);
    }
    let answer = part_1_towers.top_crate_word();
    println!("part_1: {}", answer);

    // part 2
    let mut part_2_towers = towers;
    for instruction in instructions {
        part_2_towers.exec_instruction_part_2(instruction);
    }
    println!("part_2: {}", part_2_towers.top_crate_word());

    Ok(())
}
