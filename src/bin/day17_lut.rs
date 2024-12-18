use std::{
    env,
    fs::File,
    io::{self, Write},
};

fn combo(operand: u8, a: u64, b: u64, c: u64) -> u64 {
    match operand {
        ..=3 => operand as u64,
        4 => a,
        5 => b,
        6 => c,
        _ => unreachable!(),
    }
}

fn evaluate(program: [u8; 16], mut a: u64) -> Vec<u8> {
    let mut pc = 0;
    let (mut b, mut c) = (0, 0);

    let mut out = Vec::new();

    while pc < 16 {
        let op = program[pc + 1];
        match program[pc] {
            0 => {
                assert!(op == 3);
                a /= 8;
            }
            1 => b ^= op as u64,
            2 => b = combo(op, a, b, c) & 0x7,
            3 => {
                pc = if a == 0 { pc + 2 } else { op as usize };
                continue;
            }
            4 => b ^= c,
            5 => out.push((combo(op, a, b, c) & 0x7) as u8),
            6 => b = a >> combo(op, a, b, c),
            7 => c = a >> combo(op, a, b, c),
            _ => unreachable!(),
        }

        pc += 2;
    }

    out
}

fn dfs(program: [u8; 16], a: u64, it: usize) -> Option<u64> {
    if it > 16 {
        return None;
    }

    for j in 0..8 {
        let out = evaluate(program, a * 8 + j);

        if out.len() > 16 {
            continue;
        }
        if out.as_slice() == program.as_slice() {
            return Some(a * 8 + j);
        }
        if out.as_slice() == &program[16 - out.len()..] {
            if let Some(ans) = dfs(program, a * 8 + j, it + 1) {
                return Some(ans);
            }
        }
    }

    None
}

fn generate(out: &mut Vec<u64>) {
    // [0, 1, 4, 5],
    // [0, 1, 5, 4], // invalid
    // [0, 4, 1, 5],
    // [0, 4, 5, 1], // invalid
    // [0, 5, 1, 4], // invalid
    // [0, 5, 4, 1], // invalid
    // [1, 0, 4, 5],
    // [1, 0, 5, 4], // invalid
    // [1, 4, 0, 5],
    // [1, 4, 5, 0],
    // [1, 5, 0, 4], // invalid
    // [1, 5, 4, 0], // invalid
    // [4, 0, 1, 5],
    // [4, 0, 5, 1], // invalid
    // [4, 1, 0, 5],
    // [4, 1, 5, 0],
    // [4, 5, 0, 1], // invalid
    // [4, 5, 1, 0], // invalid
    // [5, 0, 1, 4], // invalid
    // [5, 0, 4, 1], // invalid
    // [5, 1, 0, 4], // invalid
    // [5, 1, 4, 0], // invalid
    // [5, 4, 0, 1], // invalid
    // [5, 4, 1, 0], // invalid
    const ORDERS: [[u8; 4]; 8] = [
        [0, 1, 4, 5], // 0145
        [0, 4, 1, 5], // 0415
        [1, 0, 4, 5], // 1045
        [1, 4, 0, 5], // 1405
        [1, 4, 5, 0], // 1450
        [4, 0, 1, 5], // 4015
        [4, 1, 0, 5], // 4105
        [4, 1, 5, 0], // 4150
    ];

    const ORDER_IDS: [usize; 8] = [24, 21, 23, 19, 14, 17, 16, 11];

    for i in 0..=24 {
        if !ORDER_IDS.contains(&i) {
            out.extend(vec![0; 512]);
            continue;
        }

        out.extend(generate_ord(
            ORDERS[ORDER_IDS.iter().position(|&x| x == i).unwrap()],
        ));
    }
}

fn generate_ord(ord: [u8; 4]) -> Vec<u64> {
    let mut out = Vec::new();

    for bxl_1 in 0..8 {
        for bxl_2 in 0..8 {
            for bxc_op in 0..8 {
                let ord_operand: [u8; 4] = std::array::from_fn(|i| {
                    let opcode = ord[i];

                    match opcode {
                        0 => 3,
                        5 => 5,
                        1 => bxl_2,
                        4 => bxc_op,
                        _ => unreachable!(),
                    }
                });

                #[rustfmt::skip]
                    let program = [
                        2, 4,
                        1, bxl_1,
                        7, 5,
                        ord[0], ord_operand[0],
                        ord[1], ord_operand[1],
                        ord[2], ord_operand[2],
                        ord[3], ord_operand[3],
                        3, 0,
                    ];

                let ans = dfs(program, 0, 0).unwrap_or(0);
                if ans != 0 {
                    println!("{:?} {}", program, ans);
                }
                out.push(ans);
            }
        }
    }

    out
}

fn main() -> io::Result<()> {
    let dst = env::current_dir().unwrap().join("day17_lut.bin");

    let mut results = Vec::new();
    generate(&mut results);

    let mut f = File::create(&dst)?;

    let (_, bytes, _) = unsafe { results.align_to::<u8>() };
    assert_eq!(bytes.len(), 25 * 512 * 8);

    f.write(bytes)?;

    Ok(())
}
