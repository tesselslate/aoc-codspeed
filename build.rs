use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Write},
    path::Path,
};

const NUM_STONES: usize = 10000000;

fn calculate(memo: &mut HashMap<(u64, usize), u64>, stone: u64, steps: usize) -> u64 {
    if let Some(result) = memo.get(&(stone, steps)) {
        return *result;
    }

    if steps == 0 {
        return 1;
    }

    let result = if stone == 0 {
        calculate(memo, 1, steps - 1)
    } else if stone.ilog10() % 2 == 1 {
        let pow = 10u64.pow(stone.ilog10() / 2 + 1);

        let lhs = calculate(memo, stone / pow, steps - 1);
        let rhs = calculate(memo, stone % pow, steps - 1);
        lhs + rhs
    } else {
        calculate(memo, stone * 2024, steps - 1)
    };

    memo.insert((stone, steps), result);
    result
}

fn main() -> io::Result<()> {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dst = Path::new(&out_dir).join("day11_lut.bin");
    if dst.exists() {
        return Ok(());
    }

    let mut memo = HashMap::new();

    let mut results_25 = Vec::with_capacity(NUM_STONES);
    let mut results_75 = Vec::with_capacity(NUM_STONES);

    for i in 0..NUM_STONES as u64 {
        results_25.push(calculate(&mut memo, i, 25));
        results_75.push(calculate(&mut memo, i, 75));
    }

    let mut f = File::create(&dst)?;

    let (_, bytes_25, _) = unsafe { results_25.align_to::<u8>() };
    assert_eq!(bytes_25.len(), NUM_STONES * 8);

    let (_, bytes_75, _) = unsafe { results_75.align_to::<u8>() };
    assert_eq!(bytes_75.len(), NUM_STONES * 8);

    f.write(bytes_25)?;
    f.write(bytes_75)?;

    Ok(())
}
