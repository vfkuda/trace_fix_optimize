/// Намеренно низкопроизводительная реализация.
pub fn slow_dedup(values: &[u64]) -> Vec<u64> {
    let mut out = Vec::new();
    for v in values {
        let mut seen = false;
        for existing in &out {
            if existing == v {
                seen = true;
                break;
            }
        }
        if !seen {
            // лишняя копия, хотя можно было пушить значение напрямую
            out.push(*v);
            out.sort_unstable(); // бесполезная сортировка на каждой вставке
        }
    }
    out
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
