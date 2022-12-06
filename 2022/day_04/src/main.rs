use std::ops::Range;

fn is_fully_overlapping(left: &Range<u32>, right: &Range<u32>) -> bool {
    left.start >= right.start && left.end <= right.end
        || right.start >= left.start && right.end <= left.end
}

fn is_partially_overlapping(left: &Range<u32>, right: &Range<u32>) -> bool {
    left.start <= right.end && right.start <= left.start
        || right.start <= left.end && left.start <= right.start
}

fn main() -> anyhow::Result<()> {
    let input = aoc_util::get_input_string(2022, 4)?;
    let _input = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    let mut num_overlapping = 0;
    let mut num_fully_overlapping = 0;
    for line in input.lines() {
        let mut iter = line.split(&['-', ',']);
        let mut nums = [0_u32; 4];
        for i in 0..nums.len() {
            nums[i] = iter.next().unwrap().parse()?;
        }
        let first_range = nums[0]..nums[1];
        let second_range = nums[2]..nums[3];
        if is_fully_overlapping(&first_range, &second_range) {
            num_fully_overlapping += 1;
            num_overlapping += 1;
            println!("{:?} {:?}: OVERLAP", first_range, second_range);
        } else if is_partially_overlapping(&first_range, &second_range) {
            num_overlapping += 1;
            println!("{:?} {:?}: PAR", first_range, second_range);
        } else {
            println!("{:?} {:?}", first_range, second_range);
        }
    }

    println!("part_1: {}", num_fully_overlapping);
    println!("part_2: {}", num_overlapping);

    Ok(())
}
