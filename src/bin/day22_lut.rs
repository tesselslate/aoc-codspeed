use std::{
    env,
    fs::File,
    io::{self, Write},
};

#[inline(always)]
const fn hash_scalar(mut x: u32) -> u32 {
    x = ((x << 6) ^ x) & 0xFFFFFF;
    x = ((x >> 5) ^ x) & 0xFFFFFF;
    ((x << 11) ^ x) & 0xFFFFFF
}

fn main() -> io::Result<()> {
    let dst = env::current_dir().unwrap().join("day22_lut.bin");

    let mut results: Box<[u32; 16777216]> = vec![0; 16777216].try_into().unwrap();

    results.iter_mut().enumerate().for_each(|(i, x)| {
        *x = i as u32;
        for _ in 0..2000 {
            *x = hash_scalar(*x);
        }
    });

    let mut f = File::create(&dst)?;

    let (_, bytes, _) = unsafe { results.align_to::<u8>() };
    assert_eq!(bytes.len(), 16777216 * 4);

    f.write(bytes)?;

    Ok(())
}
