use nom::IResult;

// sort_version
// fn is_start_marker_naive<const N: usize>(s: &[u8]) -> bool {
//     let mut arr: [u8; N] = match s.get(0..N) {
//         Some(a) => a.try_into().unwrap(),
//         None => return false,
//     };
//     arr.sort_unstable();
//     for window in arr.windows(2) {
//         if window[0] == window[1] {
//             return false;
//         }
//     }
//     true
// }

// bitwise version
// because we are only getting alphabetic ascii characters we can just use a `u64`
// as a bitset
fn is_start_marker_bitwise<const N: usize>(s: &[u8]) -> bool {
    if s.len() < N {
        return false;
    }
    let mut bitset = 0_u64;
    for byte in s {
        bitset |= 1 << byte - b'A';
    }
    bitset.count_ones() == N as u32
}

fn find_start_marker<const N: usize>(input: &[u8]) -> IResult<&[u8], &[u8]> {
    for (maybe_marker, rest) in (0..input.len() - N).map(|skip| input[skip..].split_at(N)) {
        if is_start_marker_bitwise::<N>(maybe_marker) {
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
    let start_time = std::time::Instant::now();

    let (marker, rest) = find_start_marker::<4>(input).unwrap();
    let answer = get_answer_from_rest(input, rest);
    let marker = std::str::from_utf8(marker)?;
    println!("part_1: `{marker}` found at {}", answer);

    let (marker, rest) = find_start_marker::<14>(&input).unwrap();
    let answer = get_answer_from_rest(input, rest);
    let marker = std::str::from_utf8(marker)?;
    println!("part_2: `{marker}` found at {}", answer);

    let elapsed = start_time.elapsed();
    println!("Took {:.5} microseconds", elapsed.as_micros());

    Ok(())
}
