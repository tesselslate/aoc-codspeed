use std::{
    arch::x86_64::_pext_u32, collections::HashMap, env, fs::File, io::{self, Write}
};

use itertools::iproduct;

type Pad = ([(i8, i8); 128], (i8, i8));

const DIR_PAD: Pad = const {
    let mut pad = [(-1, -1); 128];

    pad[b'^' as usize] = (0, 1);
    pad[b'A' as usize] = (0, 2);
    pad[b'<' as usize] = (1, 0);
    pad[b'v' as usize] = (1, 1);
    pad[b'>' as usize] = (1, 2);

    (pad, (0, 0))
};

const NUM_PAD: Pad = const {
    let mut pad = [(-1, -1); 128];

    pad[b'7' as usize] = (0, 0);
    pad[b'8' as usize] = (0, 1);
    pad[b'9' as usize] = (0, 2);
    pad[b'4' as usize] = (1, 0);
    pad[b'5' as usize] = (1, 1);
    pad[b'6' as usize] = (1, 2);
    pad[b'1' as usize] = (2, 0);
    pad[b'2' as usize] = (2, 1);
    pad[b'3' as usize] = (2, 2);
    pad[b'0' as usize] = (3, 1);
    pad[b'A' as usize] = (3, 2);

    (pad, (3, 0))
};

fn path_from(src: u8, dst: u8, pad: &Pad) -> Vec<Vec<u8>> {
    fn make(a: u8, a_count: usize, b: u8, b_count: usize) -> Vec<u8> {
        let mut out = Vec::new();

        if a != 0 {
            out.extend([a].repeat(a_count));
        }
        if b != 0 {
            out.extend([b].repeat(b_count));
        }

        out
    }

    let (sr, sc) = pad.0[src as usize];
    let (dr, dc) = pad.0[dst as usize];

    let h_arrow = [b'<', 0, b'>'][((dc - sc).signum() + 1) as usize];
    let v_arrow = [b'^', 0, b'v'][((dr - sr).signum() + 1) as usize];

    let mut out = Vec::with_capacity(2);
    if pad.1 != (sr, dc) {
        out.push(make(
            h_arrow,
            (sc - dc).abs() as usize,
            v_arrow,
            (sr - dr).abs() as usize,
        ));
    }
    if pad.1 != (dr, sc) {
        out.push(make(
            v_arrow,
            (sr - dr).abs() as usize,
            h_arrow,
            (sc - dc).abs() as usize,
        ));
    }

    out
}

fn gen_pad_paths(code: &[u8]) -> Vec<Vec<u8>> {
    let code = [b'A', code[0], code[1], code[2], b'A'];

    let subpaths: Vec<Vec<Vec<u8>>> = code
        .windows(2)
        .map(|xs| path_from(xs[0], xs[1], &NUM_PAD))
        .collect();

    assert!(subpaths.len() == 4);

    let mut paths = Vec::new();

    for (a, b, c, d) in iproduct!(&subpaths[0], &subpaths[1], &subpaths[2], &subpaths[3]) {
        let mut path: Vec<u8> = Vec::new();

        path.extend(a);
        path.push(b'A');
        path.extend(b);
        path.push(b'A');
        path.extend(c);
        path.push(b'A');
        path.extend(d);
        path.push(b'A');

        paths.push(path);
    }

    paths
}

fn moves(src: u8, dst: u8, depth: usize, memo: &mut HashMap<(usize, u8, u8), u64>) -> u64 {
    if depth == 0 {
        1
    } else {
        if let Some(&cached) = memo.get(&(depth, src, dst)) {
            return cached;
        }

        let result = path_from(src, dst, &DIR_PAD)
            .iter_mut()
            .map(|path| {
                path.insert(0, b'A');
                path.push(b'A');

                path.windows(2)
                    .map(|xs| moves(xs[0], xs[1], depth - 1, memo))
                    .sum()
            })
            .min()
            .unwrap();

        memo.insert((depth, src, dst), result);
        result
    }
}

fn strcode(code: u64) -> [u8; 4] {
    [
        b'0' + (code / 100) as u8,
        b'0' + ((code % 100) / 10) as u8,
        b'0' + (code % 10) as u8,
        b'A',
    ]
}

fn pext_bits(code: u64) -> usize {
    let bytes = strcode(code);
    let u32 = u32::from_ne_bytes(bytes);

    unsafe { _pext_u32(u32, 0x000F0F0F) as usize }
}

fn complexity(code: u64, depth: usize, memo: &mut HashMap<(usize, u8, u8), u64>) -> u64 {
    gen_pad_paths(&strcode(code))
        .iter_mut()
        .map(|path| {
            path.insert(0, b'A');
            path.windows(2)
                .map(|xs| moves(xs[0], xs[1], depth, memo))
                .sum::<u64>()
        })
        .min()
        .unwrap()
        * code
}

fn generate(depth: usize) -> [u64; 4096] {
    let mut cache = HashMap::new();
    let mut out = [0; 4096];

    for code in 0..1000 {
        let result = complexity(code, depth, &mut cache);
        let bits = pext_bits(code);
        println!("{} {:?} {} {}", code, strcode(code), bits, result);

        out[bits] = result;
    }

    out
}

fn main() -> io::Result<()> {
    let dst = env::current_dir().unwrap().join("day21_lut.bin");

    let mut results: Vec<u64> = Vec::new();
    results.extend(&generate(2));
    results.extend(&generate(25));

    let mut f = File::create(&dst)?;

    let (_, bytes, _) = unsafe { results.align_to::<u8>() };
    assert_eq!(bytes.len(), 8192 * 8);

    f.write(bytes)?;

    Ok(())
}
