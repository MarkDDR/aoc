use nom::IResult;

fn is_start_marker<const N: usize>(s: &[u8]) -> bool {
    let mut arr: [u8; N] = match s.get(0..N) {
        Some(a) => a.try_into().unwrap(),
        None => return false,
    };
    arr.sort_unstable();
    for window in arr.windows(2) {
        if window[0] == window[1] {
            return false;
        }
    }
    true
}

fn find_start_marker<const N: usize>(input: &[u8]) -> IResult<&[u8], &[u8]> {
    for (maybe_marker, rest) in (0..input.len() - N).map(|skip| input[skip..].split_at(N)) {
        if is_start_marker::<N>(maybe_marker) {
            return Ok((maybe_marker, rest));
        }
    }

    panic!("No start marker")
    // Err(anyhow::anyhow!("No start marker"))
}

fn get_answer_from_rest(input: &[u8], rest: &[u8]) -> usize {
    rest.as_ptr() as usize - input.as_ptr() as usize
}

fn main() -> anyhow::Result<()> {
    // let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    let input = aoc_util::get_input_string(2022, 6)?;
    let input = input.as_bytes();

    let (marker, rest) = find_start_marker::<4>(input).unwrap();
    let answer = get_answer_from_rest(input, rest);
    let marker = std::str::from_utf8(marker)?;
    println!("part_1: `{marker}` found at {}", answer);

    let (marker, rest) = find_start_marker::<14>(&input).unwrap();
    let answer = get_answer_from_rest(input, rest);
    let marker = std::str::from_utf8(marker)?;
    println!("part_2: `{marker}` found at {}", answer);

    Ok(())
}
