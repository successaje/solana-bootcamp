// functions3.rs
// Make me compile! Execute `rustlings hint functions3` for hints :)

// I AM DONE

fn main() {
    call_this(10);
}

fn call_this(num: u32) {
    for i in 0..num {
        println!("Loop now {}", i + 1);
    }
}
