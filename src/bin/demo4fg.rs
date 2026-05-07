use broken_app::{algo, leak_buffer, normalize, sum_even};
use std::hint::black_box;

fn main() {
    for _ in 0..100_000 {
        let nums = [1, 2, 3, 4];
        let _ = black_box(sum_even(&nums));
        //
        let data = [1_u8, 0, 2, 3];
        let _ = black_box(leak_buffer(&data));
        //
        let text = " Hello World ";
        let _ = black_box(normalize(text));
        //
        let _ = black_box(algo::slow_fib(20));
        //
        let _ = black_box(algo::slow_dedup(&[1, 2, 2, 3, 1, 4, 4]));
    }
}
