unsafe fn p1_inner(input: *const u8) -> u64 {
    let mut sum = 0;

    core::arch::asm!(
    // set loop counter
        "push 320",
    "20:", // load ax
        "movzx {ax}, byte ptr [{inp}+12]",
        "movzx {b2}, byte ptr [{inp}+13]",
        "and {ax}, 0xF",
        "and {b2}, 0xF",
        "add {ax}, {ax}",
        "lea {ax}, [{ax} + 4*{ax}]",
        "add {ax}, {b2}",
    // load ay
        "movzx {ay}, byte ptr [{inp}+18]",
        "movzx {b2}, byte ptr [{inp}+19]",
        "and {ay}, 0xF",
        "and {b2}, 0xF",
        "add {ay}, {ay}",
        "lea {ay}, [{ay} + 4*{ay}]",
        "add {ay}, {b2}",
    // load bx
        "movzx {bx}, byte ptr [{inp}+33]",
        "movzx {b2}, byte ptr [{inp}+34]",
        "and {bx}, 0xF",
        "and {b2}, 0xF",
        "add {bx}, {bx}",
        "lea {bx}, [{bx} + 4*{bx}]",
        "add {bx}, {b2}",
    // load by
        "movzx {by}, byte ptr [{inp}+39]",
        "movzx {b2}, byte ptr [{inp}+40]",
        "and {by}, 0xF",
        "and {b2}, 0xF",
        "add {by}, {by}",
        "lea {by}, [{by} + 4*{by}]",
        "add {by}, {b2}",
    // load tx
        "xor {tx}, {tx}",
        "add {inp}, 51",
        "movzx {b1}, byte ptr [{inp}]",
        "movzx {b2}, byte ptr [{inp}+1]",
        "movzx {b3}, byte ptr [{inp}+2]",
        "movzx {b4}, byte ptr [{inp}+3]",
        "and {b1}, 0xF",
        "and {b2}, 0xF",
        "and {b3}, 0xF",
        "cmp {b4}, 0x2C",
        "je 23f",
        "movzx {b5}, byte ptr [{inp}+4]",
        "cmp {b5}, 0x2C",
        "je 24f",
    // load tx (5 digits)
        "and {b4}, 0xF",
        "and {b5}, 0xF",
        "imul {b1}, 10000",
        "imul {b2}, 1000",
        "imul {b3}, 100",
        "imul {b4}, 10",
        "add {tx}, {b1}",
        "add {tx}, {b2}",
        "add {tx}, {b3}",
        "add {tx}, {b4}",
        "add {tx}, {b5}",
        "add {inp}, 9",
        "jmp 30f",
    "23:", // load tx (3 digits)
        "imul {b1}, 100",
        "imul {b2}, 10",
        "add {tx}, {b1}",
        "add {tx}, {b2}",
        "add {tx}, {b3}",
        "add {inp}, 7",
        "jmp 30f",
    "24:", // load tx (4 digits)
        "and {b4}, 0xF",
        "imul {b1}, 1000",
        "imul {b2}, 100",
        "imul {b3}, 10",
        "add {tx}, {b1}",
        "add {tx}, {b2}",
        "add {tx}, {b3}",
        "add {tx}, {b4}",
        "add {inp}, 8",
    "30:", // load ty
        "xor {ty}, {ty}",
        "movzx {b1}, byte ptr [{inp}]",
        "movzx {b2}, byte ptr [{inp}+1]",
        "movzx {b3}, byte ptr [{inp}+2]",
        "movzx {b4}, byte ptr [{inp}+3]",
        "and {b1}, 0xF",
        "and {b2}, 0xF",
        "and {b3}, 0xF",
        "cmp {b4}, 0x0A",
        "je 33f",
        "movzx {b5}, byte ptr [{inp}+4]",
        "cmp {b5}, 0x0A",
        "je 34f",
    // load ty (5 digits)
        "and {b4}, 0xF",
        "and {b5}, 0xF",
        "imul {b1}, 10000",
        "imul {b2}, 1000",
        "imul {b3}, 100",
        "imul {b4}, 10",
        "add {ty}, {b1}",
        "add {ty}, {b2}",
        "add {ty}, {b3}",
        "add {ty}, {b4}",
        "add {ty}, {b5}",
        "add {inp}, 7",
        "jmp 40f",
    "33:", // load ty (3 digits)
        "imul {b1}, 100",
        "imul {b2}, 10",
        "add {ty}, {b1}",
        "add {ty}, {b2}",
        "add {ty}, {b3}",
        "add {inp}, 5",
        "jmp 40f",
    "34:", // load ty (4 digits)
        "and {b4}, 0xF",
        "imul {b1}, 1000",
        "imul {b2}, 100",
        "imul {b3}, 10",
        "add {ty}, {b1}",
        "add {ty}, {b2}",
        "add {ty}, {b3}",
        "add {ty}, {b4}",
        "add {inp}, 6",
    "40:", // calculate score and add to sum
        "mov {b1}, {ax}",
        "imul {b1}, {by}",
        "mov {b4}, {ay}",
        "imul {b4}, {bx}",
        "sub {b1}, {b4}",

        "xor rax, rax",
        "mov rax, {tx}",
        "imul rax, {by}",
        "mov rdx, {ty}",
        "imul rdx, {bx}",
        "sub rax, rdx",
        "cqo",
        "idiv {b1}",
        "test rdx, rdx",
        "jnz 50f",
        "mov {b2}, rax",

        "mov rax, {ty}",
        "imul rax, {ax}",
        "mov rdx, {tx}",
        "imul rdx, {ay}",
        "sub rax, rdx",
        "cqo",
        "idiv {b1}",
        "test rdx, rdx",
        "jnz 50f",

        "add {sum}, rax",
        "imul {b2}, 3",
        "add {sum}, {b2}",
    "50:", // loop
        "pop rax",
        "sub rax, 1",
        "push rax",
        "jnz 20b",
        "pop rax",

        sum = inout(reg) sum,
        inp = inout(reg) input => _,
        b1 = out(reg) _, // divisor
        b2 = out(reg) _, // a
        b3 = out(reg) _,
        b4 = out(reg) _,
        b5 = out(reg) _,
        ax = out(reg) _,
        ay = out(reg) _,
        bx = out(reg) _,
        by = out(reg) _,
        tx = out(reg) _,
        ty = out(reg) _,
        out("rax") _,
        out("rdx") _,
    );

    sum
}

unsafe fn p2_inner(input: *const u8) -> u64 {
    let mut sum = 0;

    core::arch::asm!(
    // set loop counter
        "push 320",
    "20:", // load ax
        "movzx {ax}, byte ptr [{inp}+12]",
        "movzx {b2}, byte ptr [{inp}+13]",
        "and {ax}, 0xF",
        "and {b2}, 0xF",
        "add {ax}, {ax}",
        "lea {ax}, [{ax} + 4*{ax}]",
        "add {ax}, {b2}",
    // load ay
        "movzx {ay}, byte ptr [{inp}+18]",
        "movzx {b2}, byte ptr [{inp}+19]",
        "and {ay}, 0xF",
        "and {b2}, 0xF",
        "add {ay}, {ay}",
        "lea {ay}, [{ay} + 4*{ay}]",
        "add {ay}, {b2}",
    // load bx
        "movzx {bx}, byte ptr [{inp}+33]",
        "movzx {b2}, byte ptr [{inp}+34]",
        "and {bx}, 0xF",
        "and {b2}, 0xF",
        "add {bx}, {bx}",
        "lea {bx}, [{bx} + 4*{bx}]",
        "add {bx}, {b2}",
    // load by
        "movzx {by}, byte ptr [{inp}+39]",
        "movzx {b2}, byte ptr [{inp}+40]",
        "and {by}, 0xF",
        "and {b2}, 0xF",
        "add {by}, {by}",
        "lea {by}, [{by} + 4*{by}]",
        "add {by}, {b2}",
    // load tx
        "xor {tx}, {tx}",
        "add {inp}, 51",
        "movzx {b1}, byte ptr [{inp}]",
        "movzx {b2}, byte ptr [{inp}+1]",
        "movzx {b3}, byte ptr [{inp}+2]",
        "movzx {b4}, byte ptr [{inp}+3]",
        "and {b1}, 0xF",
        "and {b2}, 0xF",
        "and {b3}, 0xF",
        "cmp {b4}, 0x2C",
        "je 23f",
        "movzx {b5}, byte ptr [{inp}+4]",
        "cmp {b5}, 0x2C",
        "je 24f",
    // load tx (5 digits)
        "and {b4}, 0xF",
        "and {b5}, 0xF",
        "imul {b1}, 10000",
        "imul {b2}, 1000",
        "imul {b3}, 100",
        "imul {b4}, 10",
        "add {tx}, {b1}",
        "add {tx}, {b2}",
        "add {tx}, {b3}",
        "add {tx}, {b4}",
        "add {tx}, {b5}",
        "add {inp}, 9",
        "jmp 30f",
    "23:", // load tx (3 digits)
        "imul {b1}, 100",
        "imul {b2}, 10",
        "add {tx}, {b1}",
        "add {tx}, {b2}",
        "add {tx}, {b3}",
        "add {inp}, 7",
        "jmp 30f",
    "24:", // load tx (4 digits)
        "and {b4}, 0xF",
        "imul {b1}, 1000",
        "imul {b2}, 100",
        "imul {b3}, 10",
        "add {tx}, {b1}",
        "add {tx}, {b2}",
        "add {tx}, {b3}",
        "add {tx}, {b4}",
        "add {inp}, 8",
    "30:", // load ty
        "mov rax, 10000000000000",
        "add {tx}, rax",
        "xor {ty}, {ty}",
        "movzx {b1}, byte ptr [{inp}]",
        "movzx {b2}, byte ptr [{inp}+1]",
        "movzx {b3}, byte ptr [{inp}+2]",
        "movzx {b4}, byte ptr [{inp}+3]",
        "and {b1}, 0xF",
        "and {b2}, 0xF",
        "and {b3}, 0xF",
        "cmp {b4}, 0x0A",
        "je 33f",
        "movzx {b5}, byte ptr [{inp}+4]",
        "cmp {b5}, 0x0A",
        "je 34f",
    // load ty (5 digits)
        "and {b4}, 0xF",
        "and {b5}, 0xF",
        "imul {b1}, 10000",
        "imul {b2}, 1000",
        "imul {b3}, 100",
        "imul {b4}, 10",
        "add {ty}, {b1}",
        "add {ty}, {b2}",
        "add {ty}, {b3}",
        "add {ty}, {b4}",
        "add {ty}, {b5}",
        "add {inp}, 7",
        "jmp 40f",
    "33:", // load ty (3 digits)
        "imul {b1}, 100",
        "imul {b2}, 10",
        "add {ty}, {b1}",
        "add {ty}, {b2}",
        "add {ty}, {b3}",
        "add {inp}, 5",
        "jmp 40f",
    "34:", // load ty (4 digits)
        "and {b4}, 0xF",
        "imul {b1}, 1000",
        "imul {b2}, 100",
        "imul {b3}, 10",
        "add {ty}, {b1}",
        "add {ty}, {b2}",
        "add {ty}, {b3}",
        "add {ty}, {b4}",
        "add {inp}, 6",
    "40:", // calculate score and add to sum
        "mov rax, 10000000000000",
        "add {ty}, rax",
        "mov {b1}, {ax}",
        "imul {b1}, {by}",
        "mov {b4}, {ay}",
        "imul {b4}, {bx}",
        "sub {b1}, {b4}",

        "xor rax, rax",
        "mov rax, {tx}",
        "imul rax, {by}",
        "mov rdx, {ty}",
        "imul rdx, {bx}",
        "sub rax, rdx",
        "cqo",
        "idiv {b1}",
        "test rdx, rdx",
        "jnz 50f",
        "mov {b2}, rax",

        "mov rax, {ty}",
        "imul rax, {ax}",
        "mov rdx, {tx}",
        "imul rdx, {ay}",
        "sub rax, rdx",
        "cqo",
        "idiv {b1}",
        "test rdx, rdx",
        "jnz 50f",

        "add {sum}, rax",
        "imul {b2}, 3",
        "add {sum}, {b2}",
    "50:", // loop
        "pop rax",
        "sub rax, 1",
        "push rax",
        "jnz 20b",
        "pop rax",

        sum = inout(reg) sum,
        inp = inout(reg) input => _,
        b1 = out(reg) _, // divisor
        b2 = out(reg) _, // a
        b3 = out(reg) _,
        b4 = out(reg) _,
        b5 = out(reg) _,
        ax = out(reg) _,
        ay = out(reg) _,
        bx = out(reg) _,
        by = out(reg) _,
        tx = out(reg) _,
        ty = out(reg) _,
        out("rax") _,
        out("rdx") _,
    );

    sum
}

pub fn part1(input: &str) -> u64 {
    unsafe { p1_inner(input.as_ptr()) }
}

pub fn part2(input: &str) -> u64 {
    unsafe { p2_inner(input.as_ptr()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input13.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 29201);
    }

    #[test]
    fn b() {
        assert_eq!(part2(INPUT), 104140871044942);
    }
}
