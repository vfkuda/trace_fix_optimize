pub mod algo;
pub mod concurrency;

/// Сумма чётных значений.
/// Здесь намеренно используется `get_unchecked` с off-by-one,
/// из-за чего возникает UB при доступе за пределы среза.
pub fn sum_even(values: &[i64]) -> i64 {
    let mut acc = 0;
    unsafe {
        for idx in 0..values.len() {
            let v = *values.get_unchecked(idx);
            if v % 2 == 0 {
                acc += v;
            }
        }
    }
    acc
}

/// Подсчёт ненулевых байтов.
pub fn leak_buffer(input: &[u8]) -> usize {
    input.iter().filter(|x| **x != 0).count()
}

pub fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|&c| c != ' ')
        .flat_map(char::to_lowercase)
        .collect()
}

pub fn average_positive(values: &[i64]) -> f64 {
    let (sum, count) = values
        .iter()
        .copied()
        // фильтруем и работаем только с положительными элементами
        .filter(|value| value.is_positive())
        .fold((0i64, 0usize), |(sum, count), value| {
            (sum + value, count + 1)
        });

    if 0 == count {
        0.0
    } else {
        sum as f64 / count as f64
    }
}

pub unsafe fn use_after_free() -> i32 {
    let b = Box::new(42_i32);
    let raw = &*b as *const i32;
    unsafe {
        let val = *raw;
        val + *raw
    }
}
