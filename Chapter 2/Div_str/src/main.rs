use std::arch::asm;     //  Inline Assembly
use std::env;           //  For args parsing

fn quo_rem_asm(a: i32, b: i32, quo_ptr: &mut i32, rem_ptr: &mut i32) {
    unsafe {
        let mut _r0: i32 = a;
        let mut _r1: i32 = b;
        let mut _r2: &mut i32 = quo_ptr;
        let mut _r3: &mut i32 = rem_ptr;
        asm!(
            "push   {{r4,r5}}",
            "ldr    r4,[{r0}]",
            "ldr    r5,[{r1}]",

            "sdiv   {r0},r4,r5",
            "str    {r0},[{r2}]",

            "mul    {r1},{r0},r5",
            "sub    {r2},r4,{r1}",
            
            "str    {r2},[{r3}]",

            "pop    {{r4,r5}}",
            r0 = inout(reg) _r0,
            r1 = in(reg)    _r1,
            r2 = in(reg)    _r2,
            r3 = in(reg)    _r3,
        );
    }
}


fn quo_rem(a: i32, b: i32, quo_ptr: &mut i32, rem_ptr: &mut i32) {
    *quo_ptr = a / b;
    *rem_ptr = a - (*quo_ptr * b); 
}

fn main() {
    // Args
    let args: Vec<String> = env::args().collect();
    let a :i32 = if args.len() > 1 {args[1].parse::<i32>().unwrap()} else {1919810};
    let b :i32 = if args.len() > 2 {args[2].parse::<i32>().unwrap()} else {114514};
    
    let mut quo :i32 = 0;
    let mut rem :i32 = 0;
    let quo_ptr = &mut quo;
    let rem_ptr = &mut rem;

    // quo_rem(a, b, quo_ptr, rem_ptr);
    // println!("{} {}", quo, rem);

    quo_rem_asm(a, b, quo_ptr, rem_ptr);
    println!("{} {}", quo, rem);
}

