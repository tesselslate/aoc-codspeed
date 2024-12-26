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
        "mov {i:e}, 500",
        "xor {key_idx:r}, {key_idx:r}",

    "20:",
        "vmovdqu {inp_data}, [{inp} + 6]",
        "movzx {rdata1}, byte ptr [{inp}]",
        "cmp {rdata1}, 46",
        "je 22f",
    "21:",
        "vpcmpeqb {ydata1}, {inp_data}, {eqmask_lock}",
        "vpmovmskb edi, {ydata1}",

        "mov eax, edi",
        "and eax, 0x1041041",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}], eax",

        "mov eax, edi",
        "and eax, 0x2082082",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+4], eax",

        "mov eax, edi",
        "and eax, 0x4104104",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+8], eax",

        "mov eax, edi",
        "and eax, 0x8208208",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+12], eax",

        "mov eax, edi",
        "and eax, 0x10410410",
        "popcnt eax, eax",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+16], eax",

        "add {LOCKS}, 20",
        "add {inp}, 43",

        "dec {i:e}",
        "jnz 20b",
        "jmp 30f",
    "22:",
        "mov rcx, {key_idx:r}",
        "mov rsi, {key_idx:r}",
        "and rcx, 0x3F",
        "shr rsi, 3",
        "and rsi, 0xF8",
        "mov rdx, 1",
        "shl rdx, cl",

        "vpcmpeqb {ydata1}, {inp_data}, {eqmask_key}",
        "vpmovmskb edi, {ydata1}",

        "mov eax, edi",
        "and eax, 0x1041041",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax], rdx",

        "mov eax, edi",
        "and eax, 0x2082082",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 256], rdx",

        "mov eax, edi",
        "and eax, 0x4104104",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 512], rdx",

        "mov eax, edi",
        "and eax, 0x8208208",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 768], rdx",

        "mov eax, edi",
        "and eax, 0x10410410",
        "popcnt eax, eax",
        "shl eax, 5",
        "add rax, rsi",
        "or [{KEYS} + rax + 1024], rdx",

        "inc {key_idx:e}",
        "add {inp}, 43",

        "dec {i:e}",
        "jnz 20b",
        "jmp 30f",
    "30:",
        "vmovdqu {ydata6}, [{KEYS}+160]",
        "vpor {ydata1}, {ydata6}, [{KEYS}+128]",
        "vmovdqu {ydata7}, [{KEYS}+416]",
        "vpor {ydata2}, {ydata7}, [{KEYS}+384]",
        "vmovdqu {ydata8}, [{KEYS}+672]",
        "vpor {ydata3}, {ydata8}, [{KEYS}+640]",
        "vmovdqu {ydata9}, [{KEYS}+928]",
        "vpor {ydata4}, {ydata9}, [{KEYS}+896]",
        "vmovdqu {ydata0}, [{KEYS}+1184]",
        "vpor {ydata5}, {ydata0}, [{KEYS}+1152]",

        "vmovdqu [{KEYS}+128], {ydata1}",
        "vpor {ydata6}, {ydata1}, [{KEYS}+96]",
        "vmovdqu [{KEYS}+384], {ydata2}",
        "vpor {ydata7}, {ydata2}, [{KEYS}+352]",
        "vmovdqu [{KEYS}+640], {ydata3}",
        "vpor {ydata8}, {ydata3}, [{KEYS}+608]",
        "vmovdqu [{KEYS}+896], {ydata4}",
        "vpor {ydata9}, {ydata4}, [{KEYS}+864]",
        "vmovdqu [{KEYS}+1152], {ydata5}",
        "vpor {ydata0}, {ydata5}, [{KEYS}+1120]",

        "vmovdqu [{KEYS}+96], {ydata6}",
        "vpor {ydata1}, {ydata6}, [{KEYS}+64]",
        "vmovdqu [{KEYS}+352], {ydata7}",
        "vpor {ydata2}, {ydata7}, [{KEYS}+320]",
        "vmovdqu [{KEYS}+608], {ydata8}",
        "vpor {ydata3}, {ydata8}, [{KEYS}+576]",
        "vmovdqu [{KEYS}+864], {ydata9}",
        "vpor {ydata4}, {ydata9}, [{KEYS}+832]",
        "vmovdqu [{KEYS}+1120], {ydata0}",
        "vpor {ydata5}, {ydata0}, [{KEYS}+1088]",

        "vmovdqu [{KEYS}+64], {ydata1}",
        "vpor {ydata6}, {ydata1}, [{KEYS}+32]",
        "vmovdqu [{KEYS}+320], {ydata2}",
        "vpor {ydata7}, {ydata2}, [{KEYS}+288]",
        "vmovdqu [{KEYS}+576], {ydata3}",
        "vpor {ydata8}, {ydata3}, [{KEYS}+544]",
        "vmovdqu [{KEYS}+832], {ydata4}",
        "vpor {ydata9}, {ydata4}, [{KEYS}+800]",
        "vmovdqu [{KEYS}+1088], {ydata5}",
        "vpor {ydata0}, {ydata5}, [{KEYS}+1056]",

        "vmovdqu [{KEYS}+32], {ydata6}",
        "vpor {ydata1}, {ydata6}, [{KEYS}]",
        "vmovdqu [{KEYS}+288], {ydata7}",
        "vpor {ydata2}, {ydata7}, [{KEYS}+256]",
        "vmovdqu [{KEYS}+544], {ydata8}",
        "vpor {ydata3}, {ydata8}, [{KEYS}+512]",
        "vmovdqu [{KEYS}+800], {ydata9}",
        "vpor {ydata4}, {ydata9}, [{KEYS}+768]",
        "vmovdqu [{KEYS}+1056], {ydata0}",
        "vpor {ydata5}, {ydata0}, [{KEYS}+1024]",

        "vmovdqu [{KEYS}], {ydata1}",
        "vmovdqu [{KEYS}+256], {ydata2}",
        "vmovdqu [{KEYS}+512], {ydata3}",
        "vmovdqu [{KEYS}+768], {ydata4}",
        "vmovdqu [{KEYS}+1024], {ydata5}",

        "mov {i:e}, 250",
    "40:",
        "mov eax, [{LOCKS}-20]",
        "mov ecx, [{LOCKS}-16]",
        "mov edx, [{LOCKS}-12]",
        "mov esi, [{LOCKS}-8]",
        "mov edi, [{LOCKS}-4]",

        "vmovdqu {ydata1}, [{KEYS}+rax]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+256+rcx]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+512+rdx]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+768+rsi]",
        "vpand {ydata1}, {ydata1}, [{KEYS}+1024+rdi]",

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

        LOCKS = inout(reg) LOCKS.0.as_mut_ptr() => _,
        KEYS = inout(reg) KEYS.0.as_mut_ptr() => _,
        SCRATCH = inout(reg) SCRATCH.0.as_mut_ptr() => _,
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
