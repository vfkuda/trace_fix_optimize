use std::collections::HashSet;

/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let set: HashSet<u64> = values.iter().copied().collect();
    let mut result: Vec<u64> = set.iter().copied().collect();
    result.sort_unstable();
    result
}

pub fn slow_fib(n: u64) -> u64 {
    if n == 0 {
        return 0;
    }

    let mut prev = 0;
    let mut curr = 1;

    for _ in 1..n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }

    curr
}
