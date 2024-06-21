use std::arch::asm;     //  Inline Assembly
use std::env;           //  For args parsing

fn add_sub_asm(input: i32, add: i32, sub: i32) -> i32 {
    unsafe  {
        let mut _r0:i32 = input;
        let     _r1:i32 = add;
        let     _r2:i32 = sub;
        asm!(
            "add {r0},{r0},{r1}",
            "sub {r0},{r0},{r2}",
            r0 = inout(reg) _r0,
            r1 = in(reg)    _r1,
            r2 = in(reg)    _r2,
        );
        return _r0;
    };
}

fn main() {
    // Args
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {114514};
    let add   = if args.len() > 2 {args[2].parse::<i32>().unwrap()} else {1919};
    let sub   = if args.len() > 3 {args[3].parse::<i32>().unwrap()} else {810};

    println!("Rust: {}  ASM(inline): {} ", input + add - sub, add_sub_asm(input, add, sub));
}
