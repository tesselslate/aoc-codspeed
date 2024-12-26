#![allow(static_mut_refs)]

use std::simd::u8x32;

#[inline]
#[repr(align(64))]
unsafe fn inner_p1(input: &[u8]) -> u32 {
    #[repr(align(64))]
    struct Locks([u32; 1250]);

    #[repr(align(64))]
    struct Keys([u64; 160]);

    #[repr(align(64))]
    struct Scratch([u8; 64]);

    static mut LOCKS: Locks = Locks([0; 1250]);
    static mut KEYS: Keys = Keys([0; 160]);
    static mut SCRATCH: Scratch = Scratch([0; 64]);

    let mut sum = 0;

    core::arch::asm!(
        "mov {i:e}, 500",                       // grid parse loop counter
        "xor {key_idx:r}, {key_idx:r}",         // zero key index

    "20:", // parse loop
        "vmovdqu {inp_data}, [{inp} + 6]",      // load input chunk
        "movzx {rdata1}, byte ptr [{inp}]",     // load first input byte
        "cmp {rdata1}, 46",                     // test if grid is lock or key
        "je 22f",
    "21:", // parse loop (lock)
        "vpcmpeqb {ydata1}, {inp_data}, {eqmask_lock}",
        "vpmovmskb edi, {ydata1}",

        // column 1
        "mov eax, edi",
        "and eax, 0x1041041",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}], eax",

        // column 2
        "mov eax, edi",
        "and eax, 0x2082082",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+4], eax",

        // column 3
        "mov eax, edi",
        "and eax, 0x4104104",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+8], eax",

        // column 4
        "mov eax, edi",
        "and eax, 0x8208208",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+12], eax",

        // column 5
        "mov eax, edi",
        "and eax, 0x10410410",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+16], eax",

        "dec {i:e}",
        "jz 30f",
        "add {LOCKS}, 20",
        "add {inp}, 43",
        "jmp 20b",
    "22:", // parse loop (key)
        // KEYS + (256*col) + 32*height + (8*(key_idx/64))
        // rsi = (8*(key_idx/64))
        "mov rcx, {key_idx:r}",     // copy key_idx to rcx
        "mov rsi, {key_idx:r}",     // copy key_idx to rsi
        "and rcx, 0x3F",            // store key_idx % 64 in CL (for shl)
        "shr rsi, 3",               // store u64 offset (0, 8, 16, 24) in rsi
        "and rsi, 0xF8",
        "mov rdx, 1",               // prepare OR bitmask
        "shl rdx, cl",              // store OR bitmask in rdx (1 << CL)

        "vpcmpeqb {ydata1}, {inp_data}, {eqmask_key}",
        "vpmovmskb edi, {ydata1}",

        // column 1
        "mov eax, edi",
        "and eax, 0x1041041",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax], rdx",

        // column 2
        "mov eax, edi",
        "and eax, 0x2082082",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 256], rdx",

        // column 3
        "mov eax, edi",
        "and eax, 0x4104104",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 512], rdx",

        // column 4
        "mov eax, edi",
        "and eax, 0x8208208",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 768], rdx",

        // column 5
        "mov eax, edi",
        "and eax, 0x10410410",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 1024], rdx",

        "dec {i:e}",
        "jz 30f",
        "inc {key_idx:e}",
        "add {inp}, 43",
        "jmp 20b",
    "30:", // post processing
        "vmovdqu {ydata6}, [{KEYS}+160]",
        "vmovdqu {ydata7}, [{KEYS}+416]",
        "vmovdqu {ydata8}, [{KEYS}+672]",
        "vmovdqu {ydata9}, [{KEYS}+928]",
        "vmovdqu {ydata0}, [{KEYS}+1184]",
        "vpor {ydata1}, {ydata6}, [{KEYS}+128]",
        "vpor {ydata2}, {ydata7}, [{KEYS}+384]",
        "vpor {ydata3}, {ydata8}, [{KEYS}+640]",
        "vpor {ydata4}, {ydata9}, [{KEYS}+896]",
        "vpor {ydata5}, {ydata0}, [{KEYS}+1152]",

        "vmovdqu [{KEYS}+128], {ydata1}",
        "vmovdqu [{KEYS}+384], {ydata2}",
        "vmovdqu [{KEYS}+640], {ydata3}",
        "vmovdqu [{KEYS}+896], {ydata4}",
        "vmovdqu [{KEYS}+1152], {ydata5}",
        "vpor {ydata6}, {ydata1}, [{KEYS}+96]",
        "vpor {ydata7}, {ydata2}, [{KEYS}+352]",
        "vpor {ydata8}, {ydata3}, [{KEYS}+608]",
        "vpor {ydata9}, {ydata4}, [{KEYS}+864]",
        "vpor {ydata0}, {ydata5}, [{KEYS}+1120]",

        "vmovdqu [{KEYS}+96], {ydata6}",
        "vmovdqu [{KEYS}+352], {ydata7}",
        "vmovdqu [{KEYS}+608], {ydata8}",
        "vmovdqu [{KEYS}+864], {ydata9}",
        "vmovdqu [{KEYS}+1120], {ydata0}",
        "vpor {ydata1}, {ydata6}, [{KEYS}+64]",
        "vpor {ydata2}, {ydata7}, [{KEYS}+320]",
        "vpor {ydata3}, {ydata8}, [{KEYS}+576]",
        "vpor {ydata4}, {ydata9}, [{KEYS}+832]",
        "vpor {ydata5}, {ydata0}, [{KEYS}+1088]",

        "vmovdqu [{KEYS}+64], {ydata1}",
        "vmovdqu [{KEYS}+320], {ydata2}",
        "vmovdqu [{KEYS}+576], {ydata3}",
        "vmovdqu [{KEYS}+832], {ydata4}",
        "vmovdqu [{KEYS}+1088], {ydata5}",
        "vpor {ydata6}, {ydata1}, [{KEYS}+32]",
        "vpor {ydata7}, {ydata2}, [{KEYS}+288]",
        "vpor {ydata8}, {ydata3}, [{KEYS}+544]",
        "vpor {ydata9}, {ydata4}, [{KEYS}+800]",
        "vpor {ydata0}, {ydata5}, [{KEYS}+1056]",

        "vmovdqu [{KEYS}+32], {ydata6}",
        "vmovdqu [{KEYS}+288], {ydata7}",
        "vmovdqu [{KEYS}+544], {ydata8}",
        "vmovdqu [{KEYS}+800], {ydata9}",
        "vmovdqu [{KEYS}+1056], {ydata0}",
        "vpor {ydata1}, {ydata6}, [{KEYS}]",
        "vpor {ydata2}, {ydata7}, [{KEYS}+256]",
        "vpor {ydata3}, {ydata8}, [{KEYS}+512]",
        "vpor {ydata4}, {ydata9}, [{KEYS}+768]",
        "vpor {ydata5}, {ydata0}, [{KEYS}+1024]",

        "vmovdqu [{KEYS}], {ydata1}",
        "vmovdqu [{KEYS}+256], {ydata2}",
        "vmovdqu [{KEYS}+512], {ydata3}",
        "vmovdqu [{KEYS}+768], {ydata4}",
        "vmovdqu [{KEYS}+1024], {ydata5}",

        "mov {i:e}, 250",
    "40:", // loop over all locks to sum possible pairs
        "mov eax, [{LOCKS}]",
        "mov ecx, [{LOCKS}+4]",
        "mov edx, [{LOCKS}+8]",
        "mov esi, [{LOCKS}+12]",
        "mov edi, [{LOCKS}+16]",

        "vmovdqu {ydata1}, [{KEYS}+rax]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+256+rcx]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+512+rdx]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+768+rsi]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+1024+rdi]",

        // todo: make this Not Suck
        "vmovdqu [{SCRATCH}], {ydata1}",
        "popcnt rax, [{SCRATCH}]",
        "popcnt rcx, [{SCRATCH}+8]",
        "popcnt rdx, [{SCRATCH}+16]",
        "popcnt rsi, [{SCRATCH}+24]",
        "add {sum:e}, eax",
        "add {sum:e}, ecx",
        "add {sum:e}, edx",
        "add {sum:e}, esi",

        "sub {LOCKS}, 20",
        "dec {i:e}",
        "jnz 40b",

        LOCKS = inout(reg) LOCKS.0.as_ptr() => _,
        KEYS = inout(reg) KEYS.0.as_ptr() => _,
        SCRATCH = inout(reg) SCRATCH.0.as_ptr() => _,
        inp = inout(reg) input.as_ptr() => _,
        sum = inout(reg) sum,

        i = out(reg) _,
        key_idx = out(reg) _,
        rdata1 = out(reg) _,
        out("rax") _,
        out("rcx") _,
        out("rdx") _,
        out("rsi") _,
        out("rdi") _,

        inp_data = out(ymm_reg) _,
        ydata1 = out(ymm_reg) _,
        ydata2 = out(ymm_reg) _,
        ydata3 = out(ymm_reg) _,
        ydata4 = out(ymm_reg) _,
        ydata5 = out(ymm_reg) _,
        ydata6 = out(ymm_reg) _,
        ydata7 = out(ymm_reg) _,
        ydata8 = out(ymm_reg) _,
        ydata9 = out(ymm_reg) _,
        ydata0 = out(ymm_reg) _,
        eqmask_lock = inout(ymm_reg) u8x32::splat(b'#') => _,
        eqmask_key = inout(ymm_reg) u8x32::splat(b'.') => _,

        options(nostack),
    );

    sum
}

pub fn part1(input: &str) -> u32 {
    unsafe { inner_p1(input.as_bytes()) }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../inputs/input25.txt");

    #[test]
    fn a() {
        assert_eq!(part1(INPUT), 3021);
    }
}
