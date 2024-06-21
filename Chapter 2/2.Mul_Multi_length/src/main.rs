use std::arch::asm;     //  Inline Assembly
use std::env;           //  For args parsing

fn smull_asm(input: i32, mul: i32) -> i64 {
    unsafe {
        let mut _r0: i32 = input;
        let mut _r1: i32 = mul;
        asm!(
            "smull  {r0},{r1},{r0},{r1}",
            r0 = inout(reg) _r0,            // lower
            r1 = inout(reg) _r1,            // Higher
        );

        let mut out: i64 = 0;
        out = (out | _r1 as i64) << 32;
        out = out | _r0 as i64;
        return out;
    }
}

fn umull_asm(input: i32, mul: i32) -> u64 {
    unsafe {
        let mut _r0: u32 = input as u32;
        let mut _r1: u32 = mul as u32;
        
        let mut out: u64 = 0;
        let out_ptr = &mut out;
        asm!(
            "umull  {r0},{r1},{r0},{r1}",
            "str    {r0},[{r2}]",
            "str    {r1},[{r2},#4]",
            r0 = inout(reg) _r0,
            r1 = inout(reg) _r1,
            r2 = in(reg)    out_ptr
        );
        return out;
    }
}

fn main() {
    // Args
    let args: Vec<String> = env::args().collect();
    let input :i32 = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {114514};
    let mul   :i32 = if args.len() > 2 {args[2].parse::<i32>().unwrap()} else {1919810};

    println!("Rust:       {}", input as i64 * mul as i64);
    println!("ASM SMull:  {}", smull_asm(input, mul));
    println!("ASM UMull:  {}", umull_asm(input, mul));
}
