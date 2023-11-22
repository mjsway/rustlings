// functions3.rs
//
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a
// hint.


fn main() {
    // declare val as an i32
    let val: i32 = 12;
    // shadow val as u32 so call_me will accept it
    let val: u32 = 12;
    call_me(val);
}

fn call_me(num: u32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}
