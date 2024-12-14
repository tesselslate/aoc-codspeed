use std::{
    env,
    fs::File,
    io::{self, Write},
};

fn calculate(vpat: u32, hpat: u32) -> u32 {
    let mut n = vpat;

    loop {
        n += 101;
        if n % 103 == hpat {
            return n % 10403 + 1;
        }
    }
}

fn main() -> io::Result<()> {
    let dst = env::current_dir().unwrap().join("day14_lut.bin");
    if dst.exists() {
        return Ok(());
    }

    let mut results = Vec::new();
    for vpat in 0..101u32 {
        for hpat in 0..103u32 {
            results.push(calculate(vpat, hpat));
        }
    }

    let mut f = File::create(&dst)?;

    let (_, bytes, _) = unsafe { results.align_to::<u8>() };
    assert_eq!(bytes.len(), 10403 * 4);

    f.write(bytes)?;

    Ok(())
}
