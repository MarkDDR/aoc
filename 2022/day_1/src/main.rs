fn main() -> anyhow::Result<()> {
    let input = aoc_util::get_input_string(2022, 1)?;
    // let mut elf_calories = Vec::new();
    // top 3 calories, kept sorted
    let mut top_3 = [0; 3];

    let mut current_calories = 0;
    // if the input file didn't end with a double newline, I could make sure this works
    // by chaining an empty string at the end
    for line in input.lines() {
        if line.is_empty() {
            if current_calories > top_3[0] {
                // bottom element is guaranteed to drop out
                top_3[0] = current_calories;
                top_3.sort();
            }
            // elf_calories.push(current_calories);
            current_calories = 0;
            continue;
        }

        let item_calories: u32 = line.parse()?;
        current_calories += item_calories;
    }

    // elf_calories.sort();
    // let max = elf_calories.last().unwrap();
    // let top_3_sum: u32 = elf_calories[elf_calories.len() - 3..].iter().sum();
    // println!("{:?}", &elf_calories[elf_calories.len() - 3..]);
    let max = top_3[2];
    let top_3_sum: u32 = top_3.iter().sum();
    println!("Max calorie is {}", max);
    println!("top 3 sum = {}", top_3_sum);

    Ok(())
}
