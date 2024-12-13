use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Write},
};

const NUM_STONES: usize = 1000;
const STEPS: usize = 75;

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
    let dst = env::current_dir().unwrap().join("day11_lut.bin");
    if dst.exists() {
        return Ok(());
    }

    let mut memo = HashMap::new();
    let mut results = Vec::new();

    for step in 0..=STEPS {
        for stone in 0..NUM_STONES as u64 {
            results.push(calculate(&mut memo, stone, step));
        }
    }

    let mut f = File::create(&dst)?;

    let (_, bytes, _) = unsafe { results.align_to::<u8>() };
    assert_eq!(bytes.len(), NUM_STONES * 8 * (STEPS + 1));

    f.write(bytes)?;

    Ok(())
}
