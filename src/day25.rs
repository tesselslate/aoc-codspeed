#![allow(static_mut_refs)]

use std::simd::u8x32;

#[inline]
#[repr(align(64))]
unsafe fn inner_p1(input: &[u8]) -> u32 {
    const MASKS: [[u8; 32]; 5] = const {
        let mut arrays = [[0; 32]; 5];

        let mut i = 0;
        while i < 5 {
            let mut j = 0;
            while j < 32 {
                if j >= i && (j - i) % 6 == 0 {
                    arrays[i][j] = 0xFF;
                }

                j += 1;
            }

            i += 1;
        }

        arrays
    };

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
        // column 1
        "vpand {ydata1}, {mask1}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_lock}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}], eax",

        // column 2
        "vpand {ydata1}, {mask2}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_lock}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+4], eax",

        // column 3
        "vpand {ydata1}, {mask3}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_lock}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+8], eax",

        // column 4
        "vpand {ydata1}, {mask4}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_lock}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl eax, 5",
        "mov dword ptr [{LOCKS}+12], eax",

        // column 5
        "vpand {ydata1}, {mask5}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_lock}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
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

        // column 1
        "vpand {ydata1}, {mask1}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_key}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl rax, 5",                       // multiply height by 32
        "add rax, rsi",                     // add key_idx u64 offset
        "or [{KEYS} + rax], rdx",

        // column 2
        "vpand {ydata1}, {mask2}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_key}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl rax, 5",                       // multiply height by 32
        "add rax, rsi",                     // add key_idx u64 offset
        "or [{KEYS} + rax + 256], rdx",

        // column 3
        "vpand {ydata1}, {mask3}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_key}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl rax, 5",                       // multiply height by 32
        "add rax, rsi",                     // add key_idx u64 offset
        "or [{KEYS} + rax + 512], rdx",

        // column 4
        "vpand {ydata1}, {mask4}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_key}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl rax, 5",                       // multiply height by 32
        "add rax, rsi",                     // add key_idx u64 offset
        "or [{KEYS} + rax + 768], rdx",

        // column 5
        "vpand {ydata1}, {mask5}, {inp_data}",
        "vpcmpeqb {ydata2}, {ydata1}, {eqmask_key}",
        "vpmovmskb {rdata1}, {ydata2}",
        "popcnt rax, {rdata1}",
        "shl rax, 5",                       // multiply height by 32
        "add rax, rsi",                     // add key_idx u64 offset
        "or [{KEYS} + rax + 1024], rdx",

        "dec {i:e}",
        "jz 30f",
        "inc {key_idx:e}",
        "add {inp}, 43",
        "jmp 20b",
    "30:", // post processing
        "vmovdqu {mask1}, [{KEYS}+160]",
        "vpor {ydata1}, {mask1}, [{KEYS}+128]",
        "vmovdqu {mask2}, [{KEYS}+416]",
        "vpor {ydata2}, {mask2}, [{KEYS}+384]",
        "vmovdqu {mask3}, [{KEYS}+672]",
        "vpor {ydata3}, {mask3}, [{KEYS}+640]",
        "vmovdqu {mask4}, [{KEYS}+928]",
        "vpor {ydata4}, {mask4}, [{KEYS}+896]",
        "vmovdqu {mask5}, [{KEYS}+1184]",
        "vpor {ydata5}, {mask5}, [{KEYS}+1152]",

        "vmovdqu [{KEYS}+128], {ydata1}",
        "vpor {mask1}, {ydata1}, [{KEYS}+96]",
        "vmovdqu [{KEYS}+384], {ydata2}",
        "vpor {mask2}, {ydata2}, [{KEYS}+352]",
        "vmovdqu [{KEYS}+640], {ydata3}",
        "vpor {mask3}, {ydata3}, [{KEYS}+608]",
        "vmovdqu [{KEYS}+896], {ydata4}",
        "vpor {mask4}, {ydata4}, [{KEYS}+864]",
        "vmovdqu [{KEYS}+1152], {ydata5}",
        "vpor {mask5}, {ydata5}, [{KEYS}+1120]",

        "vmovdqu [{KEYS}+96], {mask1}",
        "vpor {ydata1}, {mask1}, [{KEYS}+64]",
        "vmovdqu [{KEYS}+352], {mask2}",
        "vpor {ydata2}, {mask2}, [{KEYS}+320]",
        "vmovdqu [{KEYS}+608], {mask3}",
        "vpor {ydata3}, {mask3}, [{KEYS}+576]",
        "vmovdqu [{KEYS}+864], {mask4}",
        "vpor {ydata4}, {mask4}, [{KEYS}+832]",
        "vmovdqu [{KEYS}+1120], {mask5}",
        "vpor {ydata5}, {mask5}, [{KEYS}+1088]",

        "vmovdqu [{KEYS}+64], {ydata1}",
        "vpor {mask1}, {ydata1}, [{KEYS}+32]",
        "vmovdqu [{KEYS}+320], {ydata2}",
        "vpor {mask2}, {ydata2}, [{KEYS}+288]",
        "vmovdqu [{KEYS}+576], {ydata3}",
        "vpor {mask3}, {ydata3}, [{KEYS}+544]",
        "vmovdqu [{KEYS}+832], {ydata4}",
        "vpor {mask4}, {ydata4}, [{KEYS}+800]",
        "vmovdqu [{KEYS}+1088], {ydata5}",
        "vpor {mask5}, {ydata5}, [{KEYS}+1056]",

        "vmovdqu [{KEYS}+32], {mask1}",
        "vpor {ydata1}, {mask1}, [{KEYS}]",
        "vmovdqu [{KEYS}+288], {mask2}",
        "vpor {ydata2}, {mask2}, [{KEYS}+256]",
        "vmovdqu [{KEYS}+544], {mask3}",
        "vpor {ydata3}, {mask3}, [{KEYS}+512]",
        "vmovdqu [{KEYS}+800], {mask4}",
        "vpor {ydata4}, {mask4}, [{KEYS}+768]",
        "vmovdqu [{KEYS}+1056], {mask5}",
        "vpor {ydata5}, {mask5}, [{KEYS}+1024]",

        "vmovdqu [{KEYS}], {ydata1}",
        "vmovdqu [{KEYS}+256], {ydata2}",
        "vmovdqu [{KEYS}+512], {ydata3}",
        "vmovdqu [{KEYS}+768], {ydata4}",
        "vmovdqu [{KEYS}+1024], {ydata5}",

        "mov {i:e}, 250",
    "40:", // loop over all locks to sum possible pairs
        "mov eax, [{LOCKS}]",
        "vmovdqu {ydata1}, [{KEYS}+rax]",
        "mov ecx, [{LOCKS}+4]",
        "vpand {ydata2}, {ydata1}, [{KEYS}+256+rcx]",
        "mov edx, [{LOCKS}+8]",
        "vmovdqu {ydata3}, [{KEYS}+512+rdx]",
        "mov esi, [{LOCKS}+12]",
        "vpand {ydata4}, {ydata3}, [{KEYS}+768+rsi]",
        "mov edi, [{LOCKS}+16]",
        "vmovdqu {ydata5}, [{KEYS}+1024+rdi]",
        "vpand {ydata1}, {ydata2}, {ydata4}",
        "vpand {ydata3}, {ydata1}, {ydata5}",

        // todo: make this Not Suck
        "vmovdqu [{SCRATCH}], {ydata3}",
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
        mask1 = inout(ymm_reg) u8x32::from_array(MASKS[0]) => _,
        mask2 = inout(ymm_reg) u8x32::from_array(MASKS[1]) => _,
        mask3 = inout(ymm_reg) u8x32::from_array(MASKS[2]) => _,
        mask4 = inout(ymm_reg) u8x32::from_array(MASKS[3]) => _,
        mask5 = inout(ymm_reg) u8x32::from_array(MASKS[4]) => _,
        ydata1 = out(ymm_reg) _,
        ydata2 = out(ymm_reg) _,
        ydata3 = out(ymm_reg) _,
        ydata4 = out(ymm_reg) _,
        ydata5 = out(ymm_reg) _,
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
