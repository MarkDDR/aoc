#[derive(Debug, Copy, Clone)]
enum RPS {
    Rock,
    Paper,
    Scissors,
}

impl RPS {
    fn shape_points(&self) -> u32 {
        match self {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        }
    }

    fn round_points(&self, other: &Self) -> u32 {
        use RPS::*;
        match (self, other) {
            // draw
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => 3,
            // lose
            (Rock, Paper) | (Paper, Scissors) | (Scissors, Rock) => 0,
            // win
            (Rock, Scissors) | (Scissors, Paper) | (Paper, Rock) => 6,
        }
    }

    fn total_points(&self, other: &Self) -> u32 {
        self.shape_points() + self.round_points(other)
    }

    fn get_lose(&self) -> Self {
        match self {
            RPS::Rock => RPS::Scissors,
            RPS::Paper => RPS::Rock,
            RPS::Scissors => RPS::Paper,
        }
    }

    fn get_win(&self) -> Self {
        match self {
            RPS::Rock => RPS::Paper,
            RPS::Paper => RPS::Scissors,
            RPS::Scissors => RPS::Rock,
        }
    }
}

fn main() -> anyhow::Result<()> {
    let input = aoc_util::get_input_string(2022, 2)?;

    let mut total_part1 = 0;
    let mut total_part2 = 0;
    for line in input.lines() {
        let (other, you) = line.split_once(' ').unwrap();
        let other = match other {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            unknown => unreachable!("got `{unknown}` for other"),
        };

        let you_part_1 = match you {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            unknown => unreachable!("got `{unknown}` for you"),
        };

        let you_part_2 = match you {
            "X" => other.get_lose(),
            "Y" => other,
            "Z" => other.get_win(),
            _ => unreachable!(),
        };

        total_part1 += you_part_1.total_points(&other);
        total_part2 += you_part_2.total_points(&other);
    }
    println!("part 1: {}", total_part1);
    println!("part 2: {}", total_part2);

    Ok(())
}
