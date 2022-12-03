use std::collections::HashSet;

fn get_letter_score(c: u8) -> u32 {
    (match c {
        b'A'..=b'Z' => c - b'A' + 1 + 26,
        b'a'..=b'z' => c - b'a' + 1,
        _ => panic!("Uh oh"),
    }) as u32
}

fn main() -> anyhow::Result<()> {
    let input = aoc_util::get_input_string(2022, 3)?;
    //     let input = "vJrwpWtwJgWrhcsFMMfFFhFp
    // jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
    // PmmdzqPrVvPwwTWBwg
    // wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
    // ttgJtRGJQctTZtZT
    // CrZsJsPPZsGzwwsLwLmpwMDw";

    let mut total_score = 0;
    for line in input.lines() {
        let line = line.as_bytes();
        let split_point = line.len() / 2;
        let (left, right) = line.split_at(split_point);
        let left_set: HashSet<_> = left.iter().collect();
        let right_set: HashSet<_> = right.iter().collect();
        for item in left_set.intersection(&right_set) {
            total_score += get_letter_score(**item);
        }
    }

    println!("part_1: {}", total_score);

    let mut group_index = 0;
    let mut group_sets = [(); 3].map(|_| HashSet::new());
    let mut total_score = 0;
    for line in input.lines().map(|l| l.as_bytes()) {
        group_sets[group_index] = line.iter().copied().collect();
        group_index += 1;
        if group_index == 3 {
            let mut common_letters: HashSet<_> = group_sets[0]
                .intersection(&group_sets[1])
                .copied()
                .collect();
            common_letters = common_letters
                .intersection(&group_sets[2])
                .copied()
                .collect();
            for letter in common_letters {
                total_score += get_letter_score(letter);
            }

            group_index = 0;
        }
    }

    println!("part_2: {}", total_score);
    Ok(())
}
